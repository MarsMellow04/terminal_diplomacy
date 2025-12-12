use crate::Command;
use crate::interactive::states::show_units::{self, ShowUnitState};
use diplomacy::{ShortName, Unit, UnitPosition, UnitType};
use diplomacy::{geo::RegionKey};
use diplomacy::judge::{Rulebook, Submission, MappedMainOrder};
use diplomacy::geo::{Map, Terrain, standard_map};
use serde::Deserialize;
use serde_json::{json};
use std::collections::BTreeMap; // <-- correct map type for string->string JSON

use crate::interactive::state_machine::StateMachine;

pub enum GamePhase {
    SpringMovement,
    SpringRetreat,
    FallMovement,
    FallRetreat,
    WinterBuild,
}

#[derive(Deserialize)]
struct MovesJson {
    edition: Option<String>,
    orders: Vec<String>,
}

#[derive(Default)]
pub struct OrderCommand {
    name: Option<String>,
}

impl OrderCommand {
    pub fn new(name: Option<String>) -> Self {
        Self { name }
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

impl Command for OrderCommand {
    fn execute(&self) -> bool {
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

        // The Mahcine is finished
        println!("Final orders = {:?}", machine.data.orders);
        true
    }
}
