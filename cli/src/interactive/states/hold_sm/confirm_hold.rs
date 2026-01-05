use common::context::GameContext;

use crate::interactive::state_machine::{InputResult, MachineData, State};
use crate::interactive::states::show_units::ShowUnitState;
use crate::interactive::util::{SelectResult, finalize_order, select_from};
use crate::interactive::state_machine::UiState;

#[derive(Clone, PartialEq)]
pub struct ConfirmHold;

impl State for ConfirmHold {
    fn render(&self,  _machine: &MachineData) {}

    fn handle_input(&mut self, _input: &str, machine_data: &mut MachineData, _ctx: &GameContext) -> InputResult {
        let options = vec!["Yes", "No"];
        match select_from("Do you confirm this order?", &options) {
            SelectResult::Selected("Yes") => {
                machine_data.order_intent = Some(crate::interactive::state_machine::OrderIntent::Hold);
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


    fn next(&self, _machine_data: &mut MachineData) -> crate::interactive::state_machine::UiState {
        UiState::ShowUnit(ShowUnitState) 
    }

    fn is_terminal(&self) -> bool {
        false
    }
}
