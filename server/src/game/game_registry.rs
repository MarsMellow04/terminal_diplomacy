use std::collections::HashMap;
use uuid::Uuid;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;


use super::game_handler::GameHandler;

pub struct GameRegistry {
    games: HashMap<Uuid, GameHandler>,
}
impl GameRegistry{
    /// The Game Registry maps all GameHandlers to games at the runtime 
    /// It is teh source of truth for what games are currently being used
    pub fn new() -> Self {
        // Initalising the games 
        //  (I need to change this in a sec to the fact that it should check for games already in the db)
        Self { games: HashMap::new() }
    }

    pub fn insert(&mut self, game_handler: GameHandler) {
        // This fucntion creates a new game
        self.games.insert(game_handler.id, game_handler);

    }

    pub fn delete(&mut self, game_id: &Uuid) {
        self.games.remove(game_id);

    }

    pub fn get_game() {

    }

    pub fn get_mut_game() {

    }

    
}

pub static GAME_REGISTRY: Lazy<RwLock<GameRegistry>> = Lazy::new(|| RwLock::new(GameRegistry::new()));