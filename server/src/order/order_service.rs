use diplomacy::UnitPosition;
use diplomacy::judge::MappedMainOrder;
use uuid::Uuid;
use std::iter::Successors;
use std::sync::Arc;

use crate::game::game_handler::{self, GameHandler, JoinError, OrderError};
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

    pub async fn send_order(&self, user_id: Uuid, orders: Vec<MappedMainOrder>, game_id:&Uuid) -> Result<(), OrderError> {
        let mut registry = GAME_REGISTRY.write().await;
        // let gh: &mut GameHandler =  match registry.get_mut_game(game_id) {
        //     Some(gh) => {gh}
        //     None => {
        //         eprintln!("[GAME_SERV ERROR] Non game is found");
        //         return Err(OrderError);
        //     }
        // };

        // More rusty 
        let gh = registry
            .get_mut_game(game_id)
            .ok_or(OrderError::GameNotFound)?;

        gh.recieve_order(user_id, orders)?;
        Ok(())
    }
}
