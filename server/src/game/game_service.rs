use uuid::Uuid;
use std::sync::Arc;

use crate::game::game_handler::{self, GameHandler};
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
        game_id
    }
}