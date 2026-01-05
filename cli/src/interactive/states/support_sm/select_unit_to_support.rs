use crate::interactive::state_machine::{InputResult, MachineData, OrderDraft, OrderIntent, OrderKind, State};
use crate::interactive::states::support_sm::confirm_support::ConfirmSupport;

use common::context::GameContext;
use diplomacy::UnitPosition;

#[derive(Clone, PartialEq)]
pub struct SelectHoldToSupport;

impl State for SelectHoldToSupport {
    fn render(&self, _machine_data: &MachineData) {}

    fn handle_input(
        &mut self,
        _input: &str,
        machine_data: &mut MachineData,
        ctx: &GameContext,
    ) -> InputResult {
        let selected_unit = match machine_data.selected_unit.as_ref() {
            Some(unit) => unit,
            None => return InputResult::Back,
        };

        // 1. Find regions adjacent to the selected unit
        let map = ctx.resolve_map();
        let bordering_regions = map.find_bordering(&selected_unit.region);

        // 2. Collect adjacent units
        let adjacent_units: Vec<UnitPosition<'static, _>> = ctx
            .get_unit_positions()
            .into_iter()
            .filter(|unit| bordering_regions.contains(&&unit.region))
            .collect();

        if adjacent_units.is_empty() {
            println!("No adjacent units available to support.");
            return InputResult::Back;
        }

        // 3. Let the player select one
        match crate::interactive::util::select_from(
            "Choose a unit to support hold:",
            &adjacent_units,
        ) {
            crate::interactive::util::SelectResult::Selected(unit) => {
                machine_data.order_intent = Some(OrderIntent::SupportHold { target: unit });
                InputResult::Advance
            }
            crate::interactive::util::SelectResult::Back => InputResult::Back,
            crate::interactive::util::SelectResult::Quit => InputResult::Quit,
        }
    }

    fn next(
        &self,
        _machine_data: &mut MachineData,
    ) -> crate::interactive::state_machine::UiState {
        crate::interactive::state_machine::UiState::ConfirmSupport(ConfirmSupport)
    }

    fn is_terminal(&self) -> bool {
        false
    }
}
