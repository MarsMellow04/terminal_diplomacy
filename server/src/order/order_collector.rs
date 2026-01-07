use std::{collections::{HashMap, HashSet}, error::Error};

use uuid::Uuid;
use diplomacy::{Command, Nation, Order, Phase, UnitPosition, UnitType, geo::RegionKey, judge::{MappedBuildOrder, MappedMainOrder, MappedRetreatOrder, build::WorldState, retreat::Destinations}, order};

use crate::{data::game, game::{game_handler::OrderError, game_instance::{self, GameInstance, PendingRetreat}}};

// I think we may make a trait for this? So there a three types of order collector for each type of order 

pub trait OrderCollector<O> {
    fn submit_order(&mut self, game_instance: &GameInstance, user: Uuid, orders: Vec<O>) -> Result<Uuid, OrderError>;
    fn mark_ready(&mut self, user: Uuid);
    fn is_player_ready(&self, user: &Uuid) -> bool;
    fn all_players_ready(&self) -> bool;
    fn snapshot(&self) -> Option<String>;
    fn clear(&mut self);
}

pub fn get_order_positions(orders: &Vec<MappedMainOrder>) -> HashSet<(UnitType, RegionKey)> {
    orders
        .into_iter()
        .fold(
            HashSet::new(),
            |mut set, order| {
                set.insert((order.unit_type, order.region.clone()));
                set
            }
        )
}

pub struct MainOrderCollector {
    pub player_orders: HashMap<Uuid, Vec<MappedMainOrder>>,
    ready_players: HashMap<Uuid, bool>
}

impl MainOrderCollector {
    pub fn new() -> Self  {
        Self { player_orders: HashMap::with_capacity(7), ready_players: HashMap::with_capacity(7)}
    }

    pub fn all_orders(&self) -> Vec<MappedMainOrder> {
        self
            .player_orders
            .values()
            .flat_map(|v| v.clone())
            .collect()
    }

}
impl OrderCollector<MappedMainOrder> for MainOrderCollector {
    fn submit_order(&mut self, game_instance: &GameInstance, user: Uuid, orders: Vec<MappedMainOrder>) -> Result<Uuid, OrderError> {
        // Must be same phase
        // println!("[DEBUG] This is the current stuff in the game_instabnce: {:?}", game_instance);
        if game_instance.phase != Phase::Main {
            return Err(OrderError::WrongPhase)
        }

        let nation = game_instance.players.get(&user).ok_or(OrderError::WrongPhase)?;
        let unit_count: usize = game_instance.unit_count(nation).into();
        let order_count = orders.len();

        if unit_count != order_count {
            return Err(OrderError::InvalidOrderCount { expected: unit_count, found: order_count })
        }

        // Must be their ALL of their units
        if game_instance.find_player_units(&user) != get_order_positions(&orders) {
            return Err(OrderError::InvalidOrderPositions)
        }
        
        self.player_orders.insert(user, orders);
        self.mark_ready(user);
        Ok(user)
    }

    fn mark_ready(&mut self, user: Uuid) {
        self.ready_players.insert(user, true);
    }

    fn is_player_ready(&self, user: &Uuid) -> bool {
        self.ready_players.get(user).unwrap_or(&false).clone()
    }

    fn all_players_ready(&self) -> bool {
        if self.ready_players.len() < 7 {
            return false;
        }
        self.ready_players.values().into_iter().all(|&val| val)
    }

    fn snapshot(&self) -> Option<String> {
        let val: Vec<MappedMainOrder> = self.player_orders.values().flat_map(|v| v.clone()).collect();
        serde_json::to_string(&val).ok()
    }

    fn clear(&mut self) {
        self.player_orders.clear();
        self.ready_players.clear();
    }
}

pub struct RetreatOrderCollector {
    pub player_orders: HashMap<Uuid, Vec<MappedRetreatOrder>>,
    ready_players: HashSet<Uuid>,
}

impl RetreatOrderCollector {
    pub fn new() -> Self {
        Self {
            player_orders: HashMap::new(),
            ready_players: HashSet::new(),
        }
    }

    pub fn all_orders(&self) -> Vec<MappedRetreatOrder> {
        self.player_orders.values().flat_map(|v| v.clone()).collect()
    }

    /// Auto-ready players whose nation has NO retreating units
    pub fn pre_add_readiness(&mut self, game_instance: &GameInstance) {
        let nations_with_retreats: HashSet<&Nation> = game_instance
            .pending_retreats
            .iter()
            .map(|r| &r.nation)
            .collect();

        let auto_ready_users: HashSet<Uuid> = game_instance
            .players
            .iter()
            .filter(|(_, nation)| !nations_with_retreats.contains(nation))
            .map(|(user, _)| *user)
            .collect();

        self.ready_players.extend(auto_ready_users);
    }
}

impl OrderCollector<MappedRetreatOrder> for RetreatOrderCollector {
    fn submit_order(
        &mut self,
        game_instance: &GameInstance,
        user: Uuid,
        orders: Vec<MappedRetreatOrder>,
    ) -> Result<Uuid, OrderError> {
        if game_instance.phase != Phase::Retreat {
            return Err(OrderError::WrongPhase);
        }

        let nation = game_instance.players.get(&user).ok_or(OrderError::WrongPhase)?;

        // All retreats this player MUST resolve
        let required_retreats: Vec<&PendingRetreat> = game_instance
            .pending_retreats
            .iter()
            .filter(|r| &r.nation == nation && !r.options.is_empty())
            .collect();

        if orders.len() != required_retreats.len() {
            return Err(OrderError::InvalidOrderCount {
                expected: required_retreats.len(),
                found: orders.len(),
            });
        }

        // Regions that MUST be ordered
        let required_from: HashSet<RegionKey> =
            required_retreats.iter().map(|r| r.from.clone()).collect();

        let ordered_from: HashSet<RegionKey> =
            orders.iter().map(|o| o.region.clone()).collect();

        if required_from != ordered_from {
            return Err(OrderError::InvalidOrderPositions);
        }

        // Validate destinations
        for order in &orders {
            let retreat = required_retreats
                .iter()
                .find(|r| r.from == order.region)
                .unwrap();

            if let Some(dest) = &order.command.move_dest() {
                if !retreat.options.contains(dest) {
                    return Err(OrderError::InvalidOrderPositions);
                }
            }
        }

        self.player_orders.insert(user, orders);
        self.mark_ready(user);
        Ok(user)
    }

    fn mark_ready(&mut self, user: Uuid) {
        self.ready_players.insert(user);
    }

    fn is_player_ready(&self, user: &Uuid) -> bool {
        self.ready_players.contains(user)
    }

    fn all_players_ready(&self) -> bool {
        self.ready_players.len() == 7
    }

    fn snapshot(&self) -> Option<String> {
        serde_json::to_string(
            &self.player_orders
                .values()
                .flat_map(|v| v.clone())
                .collect::<Vec<_>>(),
        )
        .ok()
    }

    fn clear(&mut self) {
        self.player_orders.clear();
        self.ready_players.clear();
    }
}
pub struct BuildOrderCollector {
    pub player_orders: HashMap<Uuid, Vec<MappedBuildOrder>>,
    ready_players: HashSet<Uuid>,
}

impl BuildOrderCollector {
    pub fn new() -> Self {
        Self {
            player_orders: HashMap::default(),
            ready_players: HashSet::default(),
        }
    }

    /// Auto-ready players with zero builds
    pub fn pre_add_readiness(&mut self, game_instance: &GameInstance) {
    //     let auto_ready_users: HashSet<Uuid> = game_instance
    //         .players
    //         .iter()
    //         .filter(|(user, _)| game_instance.build_count(user) == 0)
    //         .map(|(user, _)| *user)
    //         .collect();

    //     self.ready_players.extend(auto_ready_users);
    }

    pub fn all_orders(&self) -> Vec<MappedBuildOrder> {
        self
            .player_orders
            .values()
            .flat_map(|v| v.clone())
            .collect()
    }
}

impl OrderCollector<MappedBuildOrder> for BuildOrderCollector {
    fn submit_order(
        &mut self,
        game_instance: &GameInstance,
        user: Uuid,
        orders: Vec<MappedBuildOrder>,
    ) -> Result<Uuid, OrderError> {

        // if game_instance.phase != Phase::Build {
        //     return Err(OrderError::WrongPhase);
        // }

        // let build_count = game_instance.build_count(&user);
        // let order_count = orders.len();

        // if build_count != order_count {
        //     return Err(OrderError::InvalidOrderCount {
        //         expected: build_count,
        //         found: order_count,
        //     });
        // }

        // // Allowed build locations for this nation
        // let allowed_builds: HashSet<RegionKey> =
        //     game_instance.allowed_build_regions(&user);

        // let ordered_regions: HashSet<RegionKey> = orders
        //     .iter()
        //     .map(|o| o.region.clone())
        //     .collect();

        // if !ordered_regions.is_subset(&allowed_builds) {
        //     return Err(OrderError::InvalidOrderPositions);
        // }

        // self.player_orders.insert(user, orders);
        // self.mark_ready(user);
        Ok(user)
    }

    fn mark_ready(&mut self, user: Uuid) {
        self.ready_players.insert(user);
    }

    fn is_player_ready(&self, user: &Uuid) -> bool {
        self.ready_players.contains(user)
    }

    fn all_players_ready(&self) -> bool {
        self.ready_players.len() == 7
    }

    fn snapshot(&self) -> Option<String> {
        let val: Vec<MappedBuildOrder> =
            self.player_orders.values().flat_map(|v| v.clone()).collect();
        serde_json::to_string(&val).ok()
    }

    fn clear(&mut self) {
        self.player_orders.clear();
        self.ready_players.clear();
    }
}