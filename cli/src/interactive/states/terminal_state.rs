use crate::interactive::state_machine::InputResult;
use crate::interactive::state_machine::MachineData;
use crate::interactive::state_machine::State;
use crate::interactive::state_machine::StateMachine;

#[derive(Clone, PartialEq)]
pub struct TerminalState;

impl State for TerminalState {
    fn render(&self, _machine: &StateMachine) {
        // Nothing to render
    }

    fn handle_input(&mut self, _input: &str, _achine_data: &mut MachineData) -> Option<InputResult>{
        None
        // no-op
    }

    fn next(self: Box<Self>, _machine: &mut StateMachine) -> Box<dyn State> {
        self // remains terminal forever
    }

    fn is_terminal(&self) -> bool {
        true
    }
}
