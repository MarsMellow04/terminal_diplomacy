use crate::interactive::state_machine::State;
use crate::interactive::state_machine::StateMachine;

pub struct TerminalState;

impl State for TerminalState {
    fn render(&self) {
        // Nothing to render
    }

    fn handle_input(&mut self, _input: &str) {
        // no-op
    }

    fn next(self: Box<Self>, _machine: &mut StateMachine) -> Box<dyn State> {
        self // remains terminal forever
    }

    fn is_terminal(&self) -> bool {
        true
    }
}
