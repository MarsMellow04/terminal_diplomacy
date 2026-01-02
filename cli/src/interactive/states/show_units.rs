use std::borrow::Cow;
use diplomacy::Unit;
use diplomacy::UnitPosition;
use crate::interactive::state_machine::MachineData;
use crate::interactive::state_machine::State;
use crate::interactive::states::show_orders::ShowOrders;
use crate::interactive::state_machine::InputResult;
use crate::interactive::state_machine::UiState;
use crate::interactive::util::SelectResult;
use crate::interactive::util::UnitAt;
use crate::interactive::util::select_from;
use crate::rules::game_context::GameContext;

#[derive(Clone, PartialEq)]
pub struct ShowUnitState;

impl State for ShowUnitState {
    fn render(&self, _machine_data: &MachineData) {}

    fn handle_input(&mut self, _input: &str, data: &mut MachineData, ctx: &GameContext) -> InputResult {
       let Some(units) = ctx.remaining_units(&data.orders) else {
           println!("User has no possible moves!");
           return InputResult::Quit;
       };

       if units.is_empty() {
            println!("User has ordered all moves!");
            return InputResult::Quit;
       }

       let display_units: Vec<UnitAt> = units.into_iter().map(Into::into).collect();

       match select_from("Select command: ", &display_units) {
            SelectResult::Selected(unit) => {
                data.selected_unit = UnitPosition::new(
                    Unit::new(Cow::Owned(ctx.user_nation.clone()), unit.0), 
                    unit.1).into();
                InputResult::Advance
            }
            SelectResult::Back => {InputResult::Back}
            SelectResult::Quit => {InputResult::Quit}
        }
    }

    fn next(&self, _machine_data: &mut MachineData) -> UiState {
        UiState::ShowOrder(ShowOrders)
    }
    
    fn is_terminal(&self) -> bool { false }
}
