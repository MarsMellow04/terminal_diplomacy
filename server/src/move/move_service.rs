use uuid::Uuid;
use std::iter::Successors;
use std::sync::Arc;

use crate::game::game_handler::{self, GameHandler, JoinError};
use crate::game::game_instance::GameInstance;
use crate::game::game_registry::GameRegistry;

use super::game_repository::GameRepository;
use super::game_registry::GAME_REGISTRY;

pub struct MoveService {
    /// This is teh move servie, unlike games it does not manually 
    /// deal with teh db trhough the repo. That is doen from the listner 
    /// It will try and deal with the submitting a move to the game_instance and
    ///  the instance will see if it is valid.
}

impl GameService {
    pub fn new() -> Self {
        Self {}
    }
}
