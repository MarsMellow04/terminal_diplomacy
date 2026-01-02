use diplomacy::{
    UnitPosition,
    geo::RegionKey,
    judge::{MappedMainOrder, Rulebook, Submission, OrderOutcome, AttackOutcome},
};

use crate::interactive::{state_machine::{InputResult, MachineData, OrderDraft, OrderKind, State}, states::support_sm::choose_support_dest::ChooseSupportUnitState};
use crate::interactive::util::{select_from, SelectResult};
use crate::rules::game_context::GameContext;

#[derive(Clone, PartialEq)]
pub struct SelectSupportedUnitState;

impl State for SelectSupportedUnitState {
    fn render(&self, _machine_data: &MachineData) {}

    fn handle_input(
        &mut self,
        _input: &str,
        machine_data: &mut MachineData,
        ctx: &GameContext,
    ) -> InputResult {
        let supported_region = match machine_data.order_draft.as_ref().and_then(|d| d.move_to.clone()) {
            Some(region) => region,
            None => return InputResult::Back,
        };

        let selected_unit = match machine_data.selected_unit.as_ref() {
            Some(unit) => unit,
            None => return InputResult::Back,
        };

        // 1. All units except the selected one
        let candidate_units: Vec<UnitPosition<'static, RegionKey>> = ctx
            .get_unit_positions()
            .into_iter()
            .filter(|u| u.region != selected_unit.region)
            .collect();

        // 2. Keep only units that can legally move to the supported region
        let possible_units: Vec<UnitPosition<'static, RegionKey>> = candidate_units
            .into_iter()
            .filter(|unit| {
                let order = MappedMainOrder::new(
                    unit.nation().clone(),
                    unit.unit.unit_type().clone(),
                    unit.region.clone(),
                    diplomacy::order::MainCommand::Move(
                        diplomacy::order::MoveCommand::new(supported_region.clone()),
                    ),
                );

                let submission =
                    Submission::with_inferred_state(&ctx.map, vec![order]);

                let outcome = submission.adjudicate(Rulebook::default());

                let is_valid = outcome
                    .all_orders_with_outcomes()
                    .next()
                    .map(|(_, result)| {
                        *result == OrderOutcome::Move(AttackOutcome::Succeeds)
                    })
                    .unwrap_or(false);
                
                is_valid
            })
            .collect();

        if possible_units.is_empty() {
            println!("No legal units can be supported for that move.");
            return InputResult::Back;
        }

        // 3. Let the user select
        match select_from("Choose a unit to support:", &possible_units) {
            SelectResult::Selected(unit) => {
                machine_data.order_draft = Some(OrderDraft {
                    kind: Some(OrderKind::SupportMove),
                    move_to: Some(supported_region.clone()),
                    target: Some(unit),
                });
                InputResult::Advance
            }
            SelectResult::Back => InputResult::Back,
            SelectResult::Quit => InputResult::Quit,
        }
    }

    fn next(
        &self,
        _machine_data: &mut MachineData,
    ) -> crate::interactive::state_machine::UiState {
        crate::interactive::state_machine::UiState::ConfrimSupportDest(ChooseSupportUnitState)
    }

    fn is_terminal(&self) -> bool {
        false
    }
}
