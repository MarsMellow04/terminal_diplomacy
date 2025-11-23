use uuid::Uuid;
use std::fmt;
use super::game_instance::GameInstance;

type UserId = Uuid;

#[derive(Debug, Clone)]
pub struct JoinError;

impl fmt::Display for JoinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This game is full, cannot join")
    }
}

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
pub fn try_join(&mut self, user_id: UserId) -> Result<(), JoinError>{
        // In the future I want tgis to be a token taht is sent with the user to prove they are logged in but I can't for nwo 
        if self.instance.is_full() {
            eprintln!("This game is full!");
            return Err(JoinError);
        }

        if self.instance.players.contains(&user_id) {
            eprintln!("This game already contains this user");
            return Err(JoinError);
        }

        self.instance.players.push(user_id);
        Ok(())
    }
}