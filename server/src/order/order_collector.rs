use std::collections::{HashMap, HashSet};

use uuid::Uuid;
use diplomacy::{Order, Phase, UnitType, geo::RegionKey, judge::MappedMainOrder, order};

use crate::game::{game_handler::OrderError, game_instance::GameInstance};

// I think we may make a trait for this? So there a three types of order collector for each type of order 
pub struct OrderCollector {
    player_orders: HashMap<Uuid, Vec<MappedMainOrder>>,
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

impl OrderCollector {
    pub fn new() -> Self  {
        Self { player_orders: HashMap::with_capacity(7) }
    }

    pub fn submit_order(&mut self, game_instance: &GameInstance, user: Uuid, orders: Vec<MappedMainOrder>) -> Result<Uuid, OrderError> {
        // Must be same phase
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
        Ok(user)
    }

    pub fn mark_ready(user: Uuid) {}

    pub fn is_player_ready(&self, _user: Uuid) -> bool {
        false
    }

    pub fn all_players_ready(&self) -> bool {
        false
    }

    pub fn snapshot() {}

    pub fn clear() {}
}