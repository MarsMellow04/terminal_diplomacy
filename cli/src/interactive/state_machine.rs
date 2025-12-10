use diplomacy::{UnitPosition, judge::MappedMainOrder, order::MainCommand};
use crate::{interactive::states::terminal_state::TerminalState, rules::order_builder::OrderBuilder};
use super::states::show_orders::PrintCommand;

pub trait State {
    /// Render the prompt for this state
    fn render(&self, state_machine: &StateMachine);
    fn handle_input(&mut self, input: &str, machine_data: &mut MachineData) -> Option<InputResult>;
    fn next(self: Box<Self>, state_machine:&mut StateMachine) -> Box<dyn State>;
    fn is_terminal(&self) -> bool;
}

pub enum InputResult {
    Continue,       // Stay in same state
    Advance,        // Move to next state
    Quit,           // Exit to terminal
}

pub struct StateMachine {
    pub data: MachineData,
    pub state: Box<dyn State>,
    pub history: Vec<Box<dyn State>>,
}

pub struct MachineData {
    /// Machine data uses strings which are then adapted into orders
    /// 
    pub all_units: Vec<String>,
    pub units_remaining: Vec<String>,
    pub selected_unit: Option<String>,
    pub orders: Vec<MappedMainOrder>,
    pub selected_order: Option<PrintCommand>,
    pub selected_destination: Option<String>,
    pub current_order: OrderBuilder
}

impl StateMachine {
    pub fn new(inital_state:Box<dyn State>, initial_units: Vec<String>) -> Self {
        Self {
            data: MachineData {
                all_units: initial_units.clone(),
                units_remaining: initial_units.clone(),
                selected_unit: None,
                orders: vec![],
                selected_order: None,
                selected_destination: None,
                current_order: OrderBuilder::new()

            },
            state: inital_state,
            history: vec![]
        }
    }

    pub fn update(&mut self, input: &str) {
        // I am unwrapping because the only time this is an option is when it is terminal
        let input_result =  self.state.handle_input(input, &mut self.data).unwrap();
        
        // Find the input results
        match input_result {
            InputResult::Continue => {}
            InputResult::Quit => {
                // Make it go to the terminal state
                self.state = Box::new(TerminalState)
            }
            InputResult::Advance => {
                let current_state = std::mem::replace(
                &mut self.state,
                Box::new(TerminalState), // temporary placeholder; replaced immediately below
                );
                self.state = current_state.next(self);
            }
        } 
    }

    pub fn is_finished(&self) -> bool {
        self.state.is_terminal()
    } 
}