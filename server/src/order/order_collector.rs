use std::{collections::{HashMap, HashSet}, error::Error};

use uuid::Uuid;
use diplomacy::{Order, Phase, UnitType, geo::RegionKey, judge::MappedMainOrder, order};

use crate::game::{game_handler::OrderError, game_instance::GameInstance};

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

        let unit_count: usize = game_instance.unit_count(&user).into();
        let order_count = orders.len();

        if unit_count != order_count {
            return Err(OrderError::IncorrectOrderCount { expected: unit_count, found: order_count })
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
    }
}