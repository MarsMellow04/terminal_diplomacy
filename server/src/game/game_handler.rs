use std::borrow::Cow;
use std::fmt;
use std::collections::{HashMap, HashSet};

use diplomacy::Command;
use uuid::Uuid;
use diplomacy::{
    Nation, Phase, Unit, UnitPosition,
    geo::RegionKey,
    judge::{
        MappedBuildOrder, MappedMainOrder, MappedRetreatOrder,
        Rulebook, Submission,
    },
    UnitPositions,
};

use crate::{
    game::game_instance::{GameInstance, PendingRetreat},
    order::order_collector::{
        MainOrderCollector, RetreatOrderCollector, BuildOrderCollector, OrderCollector,
    },
};

type UserId = Uuid;

#[derive(Debug)]
pub enum OrderError {
    WrongPhase,
    UserReadied,
    IncorrectOrderCount,
    InvalidOrderCount { expected: usize, found: usize },
    InvalidOrderPositions,
    GameNotFound
}

#[derive(Debug)]
pub enum OrderOutcome {
    Accepted,
    GameAdvanced,
}

#[derive(Debug, Clone)]
pub struct JoinError;

impl fmt::Display for JoinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "This game is full or the user has already joined")
    }
}

impl std::error::Error for JoinError {}

pub struct GameHandler {
    pub id: Uuid,
    pub instance: GameInstance,
    pub main_orders: MainOrderCollector,
    pub retreat_orders: RetreatOrderCollector,
    pub build_orders: BuildOrderCollector,
}

impl GameHandler {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            instance: GameInstance::new(),
            main_orders: MainOrderCollector::new(),
            retreat_orders: RetreatOrderCollector::new(),
            build_orders: BuildOrderCollector::new(),
        }
    }

    pub fn try_join(&mut self, user_id: UserId) -> Result<(), JoinError> {
        if self.instance.is_full() {
            return Err(JoinError);
        }
        if self.instance.players.contains_key(&user_id) {
            return Err(JoinError);
        }

        let taken: HashSet<&Nation> = self.instance.players.values().collect();

        // TODO: Make this random, but for testing it's deterministic
        let nation = ["eng", "fra", "ger", "ita", "aus", "rus", "tur"]
            .into_iter()
            .map(Nation::from)
            .find(|n| !taken.contains(n))
            .expect("No available nations, but game is not full");

        self.instance.players.insert(user_id, nation);
        Ok(())
    }

// Main

    pub fn resolve_main(&mut self) -> Result<(), OrderError> {
        let orders = self.main_orders.all_orders();
        let submission = Submission::with_inferred_state(self.instance.map_used(), orders);
        let outcome = submission.adjudicate(Rulebook::default());

        let retreat = outcome.to_retreat_start();

        // Apply successful
        let positions = owned_positions(retreat.unit_positions());
        
        // Extract owned retreat info
        let retreat_data: Vec<_> = retreat.retreat_destinations().iter()
            .map(|(pos, dests)| {
                let available: HashSet<RegionKey> = dests.available().into_iter().cloned().collect::<HashSet<_>>();
                (pos.unit.nation().clone(), pos.unit.unit_type(), pos.region.clone(), available)
            })
            .collect();

        // Drop retreat to release the immutable borrow
        drop(retreat);

        self.instance.apply_new_positions(positions.clone());
        self.instance.pending_retreats.clear();

        for (nation, unit_type, from, available) in retreat_data {
            if !available.is_empty() {
                self.instance.pending_retreats.push(PendingRetreat {
                    nation,
                    unit_type,
                    from,
                    options: available,
                });
            }
        }

        self.instance.phase = if self.instance.pending_retreats.is_empty() {
            Phase::Build
        } else {
            Phase::Retreat
        };

        self.main_orders.clear();
        Ok(())
    }

    pub fn receive_main_orders(
        &mut self,
        user_id: UserId,
        orders: Vec<MappedMainOrder>,
    ) -> Result<OrderOutcome, OrderError> {
        let ready = Self::receive_with(
            &self.instance,
            &mut self.main_orders,
            user_id,
            orders,
        )?;

        if ready {
            self.resolve_main()?;
            Ok(OrderOutcome::GameAdvanced)
        } else {
            Ok(OrderOutcome::Accepted)
        }
    }

// Retreat

    pub fn resolve_retreat(&mut self) -> Result<(), OrderError> {
        if self.instance.phase != Phase::Retreat {
            return Err(OrderError::WrongPhase);
        }

        let orders = self.retreat_orders.all_orders();

        let mut placements = Vec::new();
        let mut claims: HashMap<RegionKey, Vec<&PendingRetreat>> = HashMap::new();

        for r in &self.instance.pending_retreats {
            if let Some(ord) = orders.iter().find(|o| o.region == r.from) {
                if let Some(dest) = ord.command.move_dest() {
                    if r.options.contains(&dest) {
                        claims.entry(dest.clone()).or_default().push(r);
                    }
                }
            }
        }

        for (dest, units) in claims {
            if units.len() == 1 {
                let u = units[0];
                placements.push(UnitPosition::new(
                    Unit::new(Cow::Owned(u.nation.clone()), u.unit_type),
                    dest,
                ));
            }
        }

        self.instance.apply_new_positions(placements);
        self.instance.pending_retreats.clear();
        self.instance.phase = Phase::Build;
        self.retreat_orders.clear();
        Ok(())
    }

    pub fn receive_retreat_orders(
        &mut self,
        user_id: UserId,
        orders: Vec<MappedRetreatOrder>,
    ) -> Result<OrderOutcome, OrderError> {
        let ready = Self::receive_with(
            &self.instance,
            &mut self.retreat_orders,
            user_id,
            orders,
        )?;

        if ready {
            self.resolve_retreat()?;
            Ok(OrderOutcome::GameAdvanced)
        } else {
            Ok(OrderOutcome::Accepted)
        }
    }

// Build

    pub fn resolve_build(&mut self) -> Result<(), OrderError> {
        let orders = self.build_orders.all_orders();
        let submission = diplomacy::judge::build::Submission::new(
            self.instance.map_used(),
            &self.instance.last_owners,
            &self.instance,
            orders,
        );

        let outcome = submission.adjudicate(Rulebook::default());
        let positions: Vec<_> = outcome.to_final_unit_positions().collect();

        self.instance.apply_new_positions(positions);
        self.instance.phase = Phase::Main;
        self.build_orders.clear();
        Ok(())
    }

    pub fn receive_build_orders(
        &mut self,
        user_id: UserId,
        orders: Vec<MappedBuildOrder>,
    ) -> Result<OrderOutcome, OrderError> {
        let ready = Self::receive_with(
            &self.instance,
            &mut self.build_orders,
            user_id,
            orders,
        )?;

        if ready {
            self.resolve_retreat()?;
            Ok(OrderOutcome::GameAdvanced)
        } else {
            Ok(OrderOutcome::Accepted)
        }
    }
}


fn owned_positions<'a, I>(positions: I) -> Vec<UnitPosition<'static, RegionKey>>
where
    I: IntoIterator<Item = UnitPosition<'a, &'a RegionKey>>,
{
    positions
        .into_iter()
        .map(|p| {
            UnitPosition::new(
                Unit::new(
                    Cow::Owned(p.unit.nation().clone()),
                    p.unit.unit_type(),
                ),
                p.region.clone(),
            )
        })
        .collect()
}

impl GameHandler {
    fn receive_with<O, C>(
        instance: &GameInstance,
        collector: &mut C,
        user_id: UserId,
        orders: Vec<O>,
    ) -> Result<bool, OrderError>
    where
        C: OrderCollector<O>,
    {
        if collector.is_player_ready(&user_id) {
            return Err(OrderError::UserReadied);
        }

        collector.submit_order(instance, user_id, orders)?;
        Ok(collector.all_players_ready())
    }
}
