use crate::interactive::state_machine::{InputResult, MachineData, State};
use crate::interactive::state_machine::StateMachine;
use crate::interactive::states::terminal_state::TerminalState;
use std::fmt::{self, Display, Formatter};


pub struct SelectOtherUnitState {
}

impl SelectOtherUnitState {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl State for SelectOtherUnitState {
    fn render(&self, machine: &StateMachine) {
        println!("\nPick a unit to support (number), 'q' to quit, or 'b' to go back:\n");

    }

    fn handle_input(&mut self, input: &str, machine_data: &mut MachineData) -> Option<InputResult> {
        let trimmed = input.trim();

        match trimmed {
            "q" => return Some(InputResult::Quit),
            "b" => return Some(InputResult::Continue), // state machine handles back
            _ => {}
        }
        None
    }

    fn next(self: Box<Self>, machine: &mut StateMachine) -> Box<dyn State> {
        Box::new(TerminalState)
    }

    fn is_terminal(&self) -> bool {
        false
    }
}