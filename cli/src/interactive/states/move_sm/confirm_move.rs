use crate::interactive::state_machine::{InputResult, MachineData, State, UiState};
use crate::interactive::states::show_units::{ShowUnitState};
use crate::interactive::util::{SelectResult, finalize_order, select_from};

#[derive(Clone, PartialEq)]
pub struct ConfirmMove;

impl State for ConfirmMove {
    fn render(&self,  _data: &MachineData) {}

    fn handle_input(&mut self, _input: &str, machine_data: &mut MachineData, _ctx: &crate::rules::game_context::GameContext) -> InputResult {
        let options = vec!["Yes", "No"];
        match select_from("Do you confirm this order?", &options) {
            SelectResult::Selected("Yes") => {
                finalize_order(machine_data);
                InputResult::Advance
            }
            SelectResult::Selected(_) => {
                println!("Ok! Moving you back... ");
                InputResult::Back}

            SelectResult::Back => {InputResult::Back}
            SelectResult::Quit => {InputResult::Quit}
        }
    }

    fn next(self, _machine_data: &mut MachineData) -> crate::interactive::state_machine::UiState {
        UiState::ShowUnit(ShowUnitState) 
    }

    fn is_terminal(&self) -> bool {
        false
    }
}
