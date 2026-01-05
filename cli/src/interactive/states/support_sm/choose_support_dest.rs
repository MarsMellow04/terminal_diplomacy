use crate::interactive::state_machine::{InputResult, MachineData, State};
use crate::interactive::states::support_sm::confirm_support::ConfirmSupport;
use crate::interactive::util::{SelectResult, select_from};
use common::context::GameContext;
use diplomacy::geo::{RegionKey};
use diplomacy::judge::{AttackOutcome, MappedMainOrder, OrderOutcome, Rulebook, Submission};

#[derive(Clone, PartialEq)]
pub struct ChooseSupportUnitState;

impl State for ChooseSupportUnitState {
    fn render(&self, machine_data: &MachineData) {}

    fn handle_input(&mut self, input: &str, machine_data: &mut MachineData, ctx: &GameContext) -> InputResult {
                let supported_region = match machine_data.order_draft.as_ref().and_then(|d| d.move_to.clone()) {
            Some(region) => region,
            None => return InputResult::Back,
        };

        let selected_unit = match machine_data.selected_unit.as_ref() {
            Some(unit) => unit,
            None => return InputResult::Back,
        };

        // 1. All bordering regions that can be moved to except the selected one
        let map = ctx.resolve_map();
        let candidate_regions = map
            .find_bordering(&selected_unit.region);

        // 2. Keep only regions that can legally move to the supported region
        let possible_units: Vec<RegionKey> = candidate_regions
            .into_iter()
            .filter(|region| {
                let order = MappedMainOrder::new(
                    selected_unit.nation().clone(),
                    selected_unit.unit.unit_type().clone(),
                    selected_unit.region.clone(),
                    diplomacy::order::MainCommand::Move(
                        diplomacy::order::MoveCommand::new(supported_region.clone()),
                    ),
                );

                let submission =
                    Submission::with_inferred_state(&map, vec![order]);

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
            .cloned()
            .collect();

        if possible_units.is_empty() {
            println!("No legal units can be supported for that move.");
            return InputResult::Back;
        }

        // 3. Let the user select
        match select_from("Choose a unit to support:", &possible_units) {
            SelectResult::Selected(unit) => {
                if let Some(draft) = &machine_data.order_draft {
                    machine_data.order_intent = Some(crate::interactive::state_machine::OrderIntent::SupportMove 
                        { target: draft.target.as_ref().unwrap().clone(), to: unit });
                    InputResult::Advance
                } else {
                    InputResult::Back
                }
            }
            SelectResult::Back => InputResult::Back,
            SelectResult::Quit => InputResult::Quit,
        }
    }

    fn next(&self, machine_data: &mut MachineData) -> crate::interactive::state_machine::UiState {
        crate::interactive::state_machine::UiState::ConfirmSupport(ConfirmSupport)
    }

    fn is_terminal(&self) -> bool {
        false
    }

}