use common::context::GameContext;
use diplomacy::geo::{Map, RegionKey, Terrain, standard_map};
use diplomacy::judge::MappedMainOrder;
use diplomacy::order::{self, MoveCommand};
use diplomacy::{Nation, UnitType, UnitPosition};
use diplomacy::ShortName;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use inquire::Select;

use crate::interactive::state_machine::MachineData;
use crate::interactive::state_machine::State;
use crate::interactive::state_machine::StateMachine;
use crate::interactive::states::move_sm::confirm_move::ConfirmMove;
use crate::interactive::states::show_orders::ShowOrders;
use crate::interactive::states::terminal_state::TerminalState;
use crate::interactive::state_machine::InputResult;
use crate::interactive::states::convoy_sm::convoy::route_may_exist;

use crate::interactive::util::{SelectResult, select_from};
use crate::interactive::state_machine::UiState;
use crate::interactive::state_machine::OrderIntent;

#[derive(Clone, PartialEq)]
pub struct PickMoveState;

impl State for PickMoveState {
    fn render(&self, _machine_data: &MachineData) {
        // println!()
    }
    
    fn handle_input(&mut self, input: &str, machine_data: &mut MachineData, ctx: &GameContext) -> InputResult {
        let origin = machine_data.selected_unit.as_ref().unwrap();
        
        let mut adjacent_moves = Vec::<RegionKey>::new();
        let mut convoy_moves = Vec::<RegionKey>::new();

        let map = ctx.resolve_map();
        for region in map.regions() {
            let dest = RegionKey::from_str(&region.short_name()).unwrap();
            if dest == origin.region {
                continue;
            }

            if origin.unit.unit_type() == UnitType::Army && region.terrain() == Terrain::Sea{
                // A army cannot move to a adjacent sea
                continue;
            }

            let is_land_adjacent = map
                .find_border_between(&origin.region, &dest)
                .is_some();

            if is_land_adjacent {
                adjacent_moves.push(dest.clone());
                continue;
            }

            if origin.unit.unit_type() == UnitType::Army {
                // Create fake order
                let order = MappedMainOrder::new(
                    origin.nation().clone(),
                    origin.unit.unit_type().clone(),
                    origin.region.clone(),
                    order::MainCommand::Move(MoveCommand::new(dest.clone())),
                );

                let owned_positions = &ctx.get_unit_positions().clone();

                let borrowed_positions: Vec<UnitPosition<'_, &RegionKey>> =
                    owned_positions
                        .iter()
                        .map(UnitPosition::as_region_ref)
                        .collect();

                if route_may_exist(
                    &map, 
                    borrowed_positions, 
                    &order) {
                    convoy_moves.push(dest.clone());
                }
            }
        }

        let mut build_moves: Vec<String> = adjacent_moves
            .iter()
            .map(|region| format!("{}", region.short_name()))
            .collect();
        
        let convoy_move_strs: Vec<String> = convoy_moves
            .iter()
            .map(|region| format!("{} via convoy", region.short_name()))
            .collect();
        
        build_moves.extend(convoy_move_strs);

        // build options 
        match select_from("Choose destination:", &build_moves) {
            SelectResult::Selected(region_str) => {
                machine_data.order_intent = Some(OrderIntent::Move { to: RegionKey::from_str(&region_str).unwrap() });
                InputResult::Advance
            }
            SelectResult::Back => {InputResult::Back}
            SelectResult::Quit => {InputResult::Quit}
        }

    }

    fn next(&self, machine_data: &mut MachineData) -> UiState {
        UiState::ConfirmMove(ConfirmMove)
    }

    fn is_terminal(&self) -> bool {
        false
    }
}