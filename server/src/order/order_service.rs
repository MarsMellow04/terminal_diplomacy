use diplomacy::UnitPosition;
use diplomacy::judge::{MappedBuildOrder, MappedMainOrder, MappedRetreatOrder};
use uuid::Uuid;
use std::iter::Successors;
use std::sync::Arc;

use crate::auth::session::Session;
use crate::game::game_handler::{self, GameHandler, JoinError, OrderError, OrderOutcome};
use crate::game::game_instance::GameInstance;
use crate::game::game_registry::GameRegistry;
use crate::game::game_registry::GAME_REGISTRY;


use crate::order::order_repository::OrderRepository;

pub struct OrderService {
    /// This is teh move servie, unlike games it does not manually 
    /// deal with teh db trhough the repo. That is doen from the listner 
    /// It will try and deal with the submitting a move to the game_instance and
    ///  the instance will see if it is valid.
    order_repo: Arc<OrderRepository>
}


impl OrderService {
    pub fn new(given_repo: Arc<OrderRepository>) -> Self {
        Self {order_repo: given_repo}
    }

    pub async fn send_main_order(&self, session: &Session, orders: Vec<MappedMainOrder>) -> Result<OrderOutcome, OrderError> {
        let mut registry = GAME_REGISTRY.write().await;
        let game_id = session.current_game.unwrap();
        let user_id = session.user;
        let gh = registry
            .get_mut_game(&game_id)
            .ok_or(OrderError::GameNotFound)?;

        let res = gh.receive_main_orders(user_id, orders)?;
            Ok(res)
    }

    pub async fn send_retreat_order(&self, session: &Session, orders: Vec<MappedRetreatOrder>) -> Result<OrderOutcome, OrderError> {
        let mut registry = GAME_REGISTRY.write().await;
        let game_id = session.current_game.unwrap();
        let user_id = session.user;
        let gh = registry
            .get_mut_game(&game_id)
            .ok_or(OrderError::GameNotFound)?;

        let res = gh.receive_retreat_orders(user_id, orders)?;
            Ok(res)
    }

    pub async fn send_build_order(&self, session: &Session, orders: Vec<MappedBuildOrder>) -> Result<OrderOutcome, OrderError> {
        let mut registry = GAME_REGISTRY.write().await;
        let game_id = session.current_game.unwrap();
        let user_id = session.user;
        let gh = registry
            .get_mut_game(&game_id)
            .ok_or(OrderError::GameNotFound)?;

        let res = gh.receive_build_orders(user_id, orders)?;
            Ok(res)
    }

}
