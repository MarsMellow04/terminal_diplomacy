use diplomacy::geo::{RegionKey};
use diplomacy::{UnitPosition};

use crate::interactive::states::convoy_sm::choose_destination_of_convoy::ChooseConvoyMove;
use crate::interactive::util::{SelectResult, select_from};
use crate::interactive::state_machine::{InputResult, MachineData, State};
use crate::interactive::state_machine::{OrderDraft, OrderKind, UiState};

#[derive(Clone, PartialEq)]
pub struct ChooseConvoyUnit;

impl State for ChooseConvoyUnit {
    fn render(&self, _machine_data: &MachineData) {}
    

    fn handle_input(&mut self, _input: &str, machine_data: &mut MachineData, ctx: &crate::rules::game_context::GameContext) -> InputResult {
        let possible_regions = ctx.map
            .find_bordering(&machine_data.selected_unit.as_ref().unwrap().region);
        
        let units: Vec<UnitPosition<'static, RegionKey>>= ctx
            .get_unit_positions()
            .iter()
            .filter(|up|possible_regions.contains(&&up.region) )
            .cloned()
            .collect();

        match select_from("Choose unit to convoy:", &units) {
            SelectResult::Selected(up) => {
                machine_data.order_draft = Some(OrderDraft { kind: Some(OrderKind::Convoy), move_to: None, target: Some(up) });
                InputResult::Advance
            }
            SelectResult::Back => {InputResult::Back}
            SelectResult::Quit => {InputResult::Quit}
        }
    }
    
    fn next(&self, _machine_data: &mut MachineData) -> crate::interactive::state_machine::UiState {
        UiState::ChooseConvoyDestination(ChooseConvoyMove)
    }

    fn is_terminal(&self) -> bool {
        false
    }
}