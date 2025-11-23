use uuid::Uuid;
use super::game_instance::GameInstance;

pub struct GameHandler {
    pub id: Uuid,
    pub instance: GameInstance
}

impl GameHandler {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            instance: GameInstance::new(),
        }
    }
}