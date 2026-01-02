use std::str::FromStr;
use diplomacy::UnitPosition;
use diplomacy::geo::{RegionKey, Terrain};
use diplomacy::order::{self, MoveCommand};
use diplomacy::{ShortName,};
use diplomacy::judge::MappedMainOrder;
use crate::interactive::state_machine::OrderIntent;
use crate::interactive::states::convoy_sm::confirm_convoy::ConfirmConvoyMove;
use crate::interactive::states::convoy_sm::convoy::{route_may_exist};
use crate::interactive::util::{SelectResult, select_from};
use crate::interactive::state_machine::{InputResult, MachineData, State};
use crate::interactive::state_machine::UiState;


#[derive(Clone, PartialEq)]
pub struct ChooseConvoyMove;

impl State for ChooseConvoyMove {
    fn render(&self, _machine_data: &MachineData) {}

    fn handle_input(&mut self, _input: &str, machine_data: &mut MachineData, ctx: &crate::rules::game_context::GameContext) -> InputResult {
        // Check the adjacent  <- This will be wrong need to change
        let Some(order_draft) = machine_data.order_draft.as_ref() else {
            println!("Unreachable state found!");
            return InputResult::Quit;
        };
        let origin = order_draft.target.as_ref().expect("This should always be found from here");

        let unit_positions = &ctx.get_unit_positions().clone();
        
        let borrowed_positions: Vec<UnitPosition<'_, &RegionKey>> =
                    unit_positions
                        .iter()
                        .map(UnitPosition::as_region_ref)
                        .collect();
        
        let mut possible_moves = vec![];
        
        for region in ctx.map.regions() {
            let dest = RegionKey::from_str(&region.short_name()).unwrap();
            if (dest == origin.region) || (region.terrain() == Terrain::Sea){
                continue;
            }

            let order = MappedMainOrder::new(
                    origin.nation().clone(),
                    origin.unit.unit_type().clone(),
                    origin.region.clone(),
                    order::MainCommand::Move(MoveCommand::new(dest.clone())),
                );
                
            if route_may_exist(&ctx.map, borrowed_positions.clone(), &order) {
                possible_moves.push(dest.clone());
            }

            // Create the fake move
        }
        match select_from("Choose destination of convoy:", &possible_moves) {
            SelectResult::Selected(reg) => {
                machine_data.order_intent = Some(OrderIntent::Convoy { target: origin.clone(), to: reg });
                InputResult::Advance
            }
            SelectResult::Back => {InputResult::Back}
            SelectResult::Quit => {InputResult::Quit}
        }
    }

    fn next(self, _machine_data: &mut MachineData) -> UiState {
        UiState::ConfirmConvoy(ConfirmConvoyMove)
    }

    fn is_terminal(&self) -> bool {
        false
    }
}