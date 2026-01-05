use common::context::GameContext;
use uuid::Uuid;
use std::iter::Successors;
use std::sync::Arc;

use crate::auth::session::Session;
use crate::game::game_handler::{self, GameHandler, JoinError};
use crate::game::game_instance::GameInstance;
use crate::game::game_registry::GameRegistry;

use super::game_repository::GameRepository;
use super::game_registry::GAME_REGISTRY;

pub struct GameService {
    game_repo: Arc<GameRepository>
}

impl GameService {
    pub fn new(given_repo: Arc<GameRepository>) -> Self {
        Self {game_repo: given_repo}
    }

    pub async fn create_game(&self) -> Uuid {
        let mut registry = GAME_REGISTRY.write().await;
        // Create new hadler for the new game
        let handler: GameHandler = GameHandler::new();
        let game_id: Uuid = handler.id;
        // Runtime allocation
        registry.insert(handler);
        
        // Db allocation
        let db_result = self.game_repo.insert_game(game_id).await;
        match db_result {
            Ok(()) => {
                println!("Result is a success! Added to db");
            }
            Err(e) => {
                eprintln!("Error failed to add to databse!: {e}");
                registry.delete(&game_id);
            }
        }
        // TODO: This doesnt make sense if the game has been deleted from runtime, it will create a key error
        game_id
    }

    pub async fn join_game(&self, given_id: &Uuid, user_id: Uuid) -> Result<(), JoinError> {
        // Join a game using by finding if the game exists, afterwars then update it
        let mut registry = GAME_REGISTRY.write().await;
        // Find game:
        let gh: &mut GameHandler = match registry.get_mut_game(given_id) {
            Some(gh) => {gh}
            None => {
                eprintln!("[GAME_SERV_ERROR] Failed to find game! ");
                // Maybe add more here 
                return Err(JoinError);
            }
        };

        match gh.try_join(user_id) {
            Err(e) => {
                eprintln!("[GAME_SERV_ERROR] Failed to join game! {e}");
                println!("Please try again to join game");
            }
            Ok(()) => {
                println!("Successfully joined game!");
            }
        };

        println!("[DEBUG] Current users now in game: {:?}", gh.instance.players);
        
        Ok(())

    }

    pub async fn get_game_state(&self, session: &Session) -> Result<GameContext, String>{
        let registry = GAME_REGISTRY.read().await;
        let gh: &GameHandler = match registry.get_game(&session.current_game.unwrap()) {
            Some(gh) => {gh}
            None => {
                eprintln!("[GAME_SERV_ERROR] Failed to find game! ");
                // Maybe add more here 
                return Err("No game found".to_string());
            }
        };
        gh
            .instance
            .to_context_for(&session.user)
            .ok_or("Cannot convert instance into context".to_string())
    }
}