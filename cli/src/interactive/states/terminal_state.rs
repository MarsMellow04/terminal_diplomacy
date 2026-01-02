use crate::interactive::state_machine::InputResult;
use crate::interactive::state_machine::MachineData;
use crate::interactive::state_machine::State;
use crate::interactive::state_machine::StateMachine;
use crate::interactive::state_machine::UiState;

#[derive(Clone, PartialEq)]
pub struct TerminalState;

impl State for TerminalState {
    fn render(&self, machine_data: &MachineData) {}

    fn handle_input(&mut self, input: &str, machine_data: &mut MachineData, ctx: &crate::rules::game_context::GameContext) -> InputResult {
        InputResult::Quit
    }

    fn next(&self, machine_data: &mut MachineData) -> crate::interactive::state_machine::UiState {
        UiState::Terminal(TerminalState)
    }

    fn is_terminal(&self) -> bool {
        true
    }
}
