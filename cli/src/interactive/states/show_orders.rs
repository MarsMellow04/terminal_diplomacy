use diplomacy::{Unit,UnitPosition, UnitType};
use diplomacy::geo::RegionKey;
use diplomacy::order::{MainCommand, MoveCommand};

use crate::interactive::state_machine::State;
use crate::interactive::state_machine::StateMachine;
use crate::interactive::states::terminal_state::TerminalState;
use std::fmt::{self, Display, Formatter};


enum  PrintCommand {
   Hold,
   Support,
   Move,
   Convoy
}

impl Display for PrintCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let text = match self {
            PrintCommand::Hold    => "Hold",
            PrintCommand::Support => "Support",
            PrintCommand::Move    => "Move",
            PrintCommand::Convoy  => "Convoy",
        };

        write!(f, "{text}")
    }
}

pub struct ShowOrders {
    given_unit_pos: String,
    needs_restart: bool
}

impl ShowOrders {
    pub fn new(given_unit_pos: String) -> Self {
        Self { given_unit_pos, needs_restart: false}
    }
}

impl State for ShowOrders {
    fn render(&self) {
        println!("\nEnter a number or 'q' to quit:");
        // What we need to do now is make it that depending on the unit_type given
        let unit_type:UnitType = self.given_unit_pos
            .parse::<UnitPosition<'_, RegionKey>>()
            .unwrap()
            .unit
            .unit_type();

        let possible_commands: Vec<PrintCommand> = match unit_type {
            UnitType::Army => {vec![
                PrintCommand::Hold, 
                PrintCommand::Support,
                PrintCommand::Move]
            }
            UnitType::Fleet => {vec![
                PrintCommand::Hold,
                PrintCommand::Support,
                PrintCommand::Move,
                PrintCommand::Convoy]
            }
        };

        // Get current units
        if self.needs_restart {
            println!("Please re-pick!");
        }
        else {
            println!("\nEnter a number or 'q' to quit:");
            println!("\n\n Pick a order: \n\n");
            for (i, command) in possible_commands.iter().enumerate() {
                println!("{} ) {}", i + 1, command);
            }
        }
    }

    fn handle_input(&mut self, input: &str) {
    }

    fn next(self: Box<Self>, machine: &mut StateMachine) -> Box<dyn State> {
        Box::new(TerminalState)

    }

    fn is_terminal(&self) -> bool {
        false
    }
}
