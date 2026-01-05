use diplomacy::{Nation, judge::MappedMainOrder};
use rand::{seq::IndexedRandom};
use uuid::Uuid;
use std::{fmt::{self, format}, iter};
use crate::order::order_collector::{self, OrderCollector};

use super::game_instance::GameInstance;

type UserId = Uuid;

#[derive(Debug, Clone)]
pub struct JoinError;

impl fmt::Display for JoinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This game is full, cannot join")
    }
}

#[derive(Debug)]
pub enum OrderError {
    GameNotFound,
    WrongPhase,
    IncorrectOrderCount {
        expected: usize,
        found: usize,
    },
    InvalidOrderPositions,
    UserReadied,
}

#[derive(Debug)]
pub enum OrderOutcome {
    Accepted,              // Order stored, waiting for others
    PhaseResolved,         // Orders resolved, game state changed
    GameAdvanced,          // Phase resolved AND game advanced
}


impl fmt::Display for OrderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderError::GameNotFound =>
                write!(f, "Game not found"),

            OrderError::WrongPhase =>
                write!(f, "Orders cannot be submitted in the current phase"),

            OrderError::IncorrectOrderCount { expected, found } =>
                write!(
                    f,
                    "Incorrect number of orders: expected {}, found {}",
                    expected, found
                ),

            OrderError::InvalidOrderPositions =>
                write!(f, "Orders do not match unit positions"),
            
            OrderError::UserReadied => 
                write!(f, "User has already readied, cannot add another order.")
        }
    }
}



pub struct GameHandler {
    pub id: Uuid,
    pub instance: GameInstance,
    pub order_collector: OrderCollector
}

impl GameHandler {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            instance: GameInstance::new(),
            order_collector: OrderCollector::new(),
            
        }
    }
pub fn try_join(&mut self, user_id: UserId) -> Result<(), JoinError>{
        // In the future I want tgis to be a token taht is sent with the user to prove they are logged in but I can't for nwo 
        if self.instance.is_full() {
            eprintln!("This game is full!");
            return Err(JoinError);
        }

        if self.instance.players.contains_key(&user_id) {
            eprintln!("This game already contains this user");
            return Err(JoinError);
        }

        let in_use_nations: Vec<&Nation> = self.instance.players.values().into_iter().collect();

        // TODO: This is such messy rust code but i will deal with it later
        let possible_nation: Vec<Nation> = vec!["eng","fra","ger","ita","rus","tur","aus"]
            .into_iter()
            .filter_map(|nat| {
                let nat = Nation::from(nat);
                if !(in_use_nations.contains(&&nat)) {
                    Some(nat)
                } else {
                    None
                }
            })
            .collect();
        
        let nation = possible_nation.choose(&mut rand::rng()).unwrap().clone();
        self.instance.players.insert(user_id, nation);
        Ok(())
    }

    pub fn resolve_orders(&mut self) -> Result<(), OrderError> {
        Ok(())
    }

    pub fn recieve_order(&mut self, user_id: UserId, orders: Vec<MappedMainOrder>) -> Result<OrderOutcome, OrderError>{
        // Do other crap
        // Check if the user has already readied
        if self.order_collector.is_player_ready(user_id) {
            eprintln!("This user has already readied, cannot add another order");
            return Err(OrderError::UserReadied);
        }

        self.order_collector.submit_order(&self.instance, user_id, orders)?;
        
        // Added, check if everyone has now added 
        if self.order_collector.all_players_ready(){
            // This is the implicit check, we need to implementa timed checkig process also
            self.resolve_orders()?;
            return Ok(OrderOutcome::GameAdvanced);
        }

        println!("[DEBUG] Order has been accepted showcasing current state sitation, orders: {:?}\n", self.order_collector.player_orders);

        Ok(OrderOutcome::Accepted)
    }
}