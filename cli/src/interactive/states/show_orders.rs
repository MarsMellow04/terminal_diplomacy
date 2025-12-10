use diplomacy::{Unit,UnitPosition, UnitType};
use diplomacy::geo::RegionKey;
use diplomacy::order::{MainCommand, MoveCommand};

use crate::interactive::state_machine::{InputResult, MachineData, State};
use crate::interactive::state_machine::StateMachine;
use crate::interactive::states::hold_sm::confirm_hold::ConfirmHold;
use crate::interactive::states::move_sm::pick_move::PickMoveState;
use crate::interactive::states::support_sm::choose_support_dest::ChooseSupportUnitState;
use crate::interactive::states::terminal_state::TerminalState;
use std::fmt::{self, Display, Formatter};

use num_enum::TryFromPrimitive;

#[derive(TryFromPrimitive)]
#[repr(usize)]
pub enum  PrintCommand {
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

fn get_possible_commands(unit_type: UnitType ) -> Vec<PrintCommand>{
    match unit_type {
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
        }
}

pub struct ShowOrders {
    given_unit_pos: String,
    possible_commands: Vec<PrintCommand>,
    selected_order: Option<PrintCommand>
}

impl ShowOrders {
    pub fn new(given_unit_pos: String) -> Self {
        Self { 
            given_unit_pos, 
            possible_commands: vec![],
            selected_order: None
        }
    }
}

impl State for ShowOrders {
    fn render(&self,  _machine: &StateMachine) {
        // What we need to do now is make it that depending on the unit_type given
        let unit_type:UnitType = self.given_unit_pos
            .parse::<UnitPosition<'_, RegionKey>>()
            .unwrap()
            .unit
            .unit_type();

        let possible_commands: Vec<PrintCommand> = get_possible_commands(unit_type);
        println!("\nEnter a number or 'q' to quit:");
        println!("\n\n Pick a order: \n\n");
        for (i, command) in possible_commands.iter().enumerate() {
            println!("{} ) {}", i + 1, command);
        }
    }

    fn handle_input(&mut self, input: &str, _machine_data: &mut MachineData) -> Option<InputResult> {
        match input.trim() {
            "q" => return Some(InputResult::Quit),
            "b" => return Some(InputResult::Continue), 
            _ => {}
        }
        let unit_type:UnitType = self.given_unit_pos
            .parse::<UnitPosition<'_, RegionKey>>()
            .unwrap()
            .unit
            .unit_type();
        self.possible_commands = get_possible_commands(unit_type);
        let index = match input.parse::<usize>() {
            Ok(n) if n > 0 && n <= self.possible_commands.len() => n - 1,
            _ => return Some(InputResult::Continue),
        };
        self.selected_order = Some(PrintCommand::try_from_primitive(index).expect("Should be good"));
        Some(InputResult::Advance)

    }

    fn next(self: Box<Self>, machine: &mut StateMachine) -> Box<dyn State> {
        machine.data.selected_order = self.selected_order;
        match machine.data.selected_order.as_ref().unwrap() {
            PrintCommand::Move => {Box::new(PickMoveState::new())}
            PrintCommand::Hold => {Box::new(ConfirmHold::new())}
            PrintCommand::Support => {Box::new(ChooseSupportUnitState::new())}
            _ => {Box::new(TerminalState)}
        }
        
    }

    fn is_terminal(&self) -> bool {
        false
    }
}
