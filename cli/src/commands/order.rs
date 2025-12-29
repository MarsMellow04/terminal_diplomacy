use crate::auth::session::SessionKeeper;
use crate::commands::util::{Client, Command, CommandError};
use crate::interactive::states::show_units::{self, ShowUnitState};
use diplomacy::{ShortName, Unit, UnitPosition, UnitType};
use diplomacy::{geo::RegionKey};
use diplomacy::judge::{Rulebook, Submission, MappedMainOrder};
use diplomacy::geo::{Map, Terrain, standard_map};
use serde::Deserialize;
use serde_json::{json};
use std::collections::BTreeMap;
use std::net::TcpStream; // <-- correct map type for string->string JSON

use crate::interactive::state_machine::StateMachine;

#[derive(Default)]
pub struct OrderCommand<C: Client, S: SessionKeeper>{
    client: C, 
    session: S,
    name: Option<String>,
    game_id: String
}

impl <C: Client, S: SessionKeeper> OrderCommand<C,S> {
    pub fn new(client: C, session: S, name: Option<String>, game_id: String) -> Self {
        Self { client, session, name, game_id }
    }

    fn parse_flags(&self) -> Option<String> {
        println!("Checking flags... ");
        if self.name.is_none() {
            println!("Meowza the flags are empty");
            return None
        }
        None
    }

}

impl <C: Client, S: SessionKeeper> OrderCommand<C,S> {
    pub fn execute(&mut self) -> Result<(), CommandError> {
        if self.parse_flags().is_some() {
            println!("Flags have been added! will skip interactive method")
        }
        use std::io::{self, Write};

        // Creates and starts the StateMachine
        let mut machine = StateMachine::new(
            Box::new(ShowUnitState::new()),
            vec!["FRA: F eng", "FRA: A par","FRA: A mos", "FRA: A bur", "FRA: A pic"] 
                .into_iter()
                .map(|str | str.to_string())
                .collect()
        );
        while !machine.is_finished() {
            machine.state.render(&machine);

            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            machine.update(input.trim());
        }
        
        let session_token = self
            .session
            .load()
            .ok_or(CommandError::NoSessionToken)?;

        let Ok(orders) = serde_json::to_string(&machine.data.orders) else {
            return Err(CommandError::WriteFailure)
        };

        // JOIN;<session_id>;<orders>\n
        let msg = format!("ORDER;MAIN;{};{}",session_token, orders);
        self.client.send(&msg)
    }
}
