use crate::interactive::state_machine::{InputResult, MachineData, State, StateMachine};
use crate::interactive::states::show_orders::SupportType;
use crate::interactive::states::support_sm::select_supported_unit::SelectSupportedUnitState;
use crate::interactive::states::support_sm::select_unit_to_support::SelectHoldToSupport;
use crate::interactive::states::terminal_state::TerminalState;
use crate::interactive::states::support_sm::choose_support_dest::ChooseSupportUnitState;

use inquire::{InquireError, Select};

pub struct SelectSupportTypeState {
}

impl SelectSupportTypeState {
    pub fn new() -> Self {
        Self { }
    }
}

fn map_to_enum(option: &str) -> Result<SupportType, ()>{
    match option {
        "Support Hold" => return Ok(SupportType::SupportHold),
        "Support Move" => return Ok(SupportType::SupportMove),
        _ => return Err(())
    }
}

impl State for SelectSupportTypeState {
    fn render(&self, state_machine: &StateMachine) {}
    fn handle_input(&mut self, input: &str, machine_data: &mut MachineData) -> Option<InputResult> {
        let options: Vec<&str> = vec!["Support Hold", "Support Move"];

        let ans: Result<&str, InquireError> = Select::new("What type of support order do you wish to make?", options).prompt();

        match ans {
            Ok(choice) => {
                machine_data.selected_support = Some(map_to_enum(choice).expect("This should never be hit"));
                return Some(InputResult::Advance);
            }
            Err(_) => return Some(InputResult::Quit),
        }   
    }
    fn next(self: Box<Self>, state_machine:&mut StateMachine) -> Box<dyn State> {
        match state_machine.data.selected_support.as_ref() {
            Some(SupportType::SupportHold) => {return Box::new(SelectHoldToSupport::new())}
            Some(SupportType::SupportMove) => {return Box::new(ChooseSupportUnitState::new())}
            None => {return Box::new(TerminalState)}
        }
        
    }
    fn is_terminal(&self) -> bool {
        false
    }
}