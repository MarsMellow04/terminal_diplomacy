use diplomacy::{UnitPosition, judge::MappedMainOrder};
use crate::interactive::states::terminal_state::TerminalState;

pub trait State {
    /// Render the prompt for this state
    fn render(&self);
    fn handle_input(&mut self, input: &str);
    fn next(self: Box<Self>, state_machine:&mut StateMachine) -> Box<dyn State>;
    fn is_terminal(&self) -> bool;
}

pub struct StateMachine {
    pub data: MachineData,
    pub state: Box<dyn State>,
}

pub struct MachineData {
    /// Machine data uses strings which are then adapted into orders
    pub units_remaining: Vec<String>,
    pub selected_unit: Option<String>,
    pub orders: Vec<String>,
}

impl StateMachine {
    pub fn new(inital_state:Box<dyn State> ) -> Self {
        Self {
            data: MachineData {
                units_remaining: vec![],
                selected_unit: None,
                orders: vec![]
            },
            state: inital_state
        }
    }

    pub fn update(&mut self, input: &str) {
        self.state.handle_input(input);
        let current_state = std::mem::replace(
            &mut self.state,
            Box::new(TerminalState), // temporary placeholder; replaced immediately below
        );
        self.state = current_state.next(self);
    }

    pub fn is_finished(&self) -> bool {
        self.state.is_terminal()
    } 
}