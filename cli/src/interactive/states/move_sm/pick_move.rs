use diplomacy::geo::{Map, RegionKey, Terrain, standard_map};
use diplomacy::judge::MappedMainOrder;
use diplomacy::order::{self, MoveCommand};
use diplomacy::{Nation, UnitType, UnitPosition};
use diplomacy::ShortName;
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


pub struct PickMoveState;

impl PickMoveState {
    pub fn new() -> Self {
        Self {}
    }
}

impl State for PickMoveState {
    fn render(&self, _sm: &StateMachine) {}

    fn handle_input(&mut self, _input: &str, machine_data: &mut MachineData) -> Option<InputResult> {
        let map = standard_map();

        // Origin province where the selected army is located
        let Some(unit_pos_str) = machine_data.selected_unit.as_ref() else {
            println!("No selected origin.");
            return Some(InputResult::Quit);
        };
        let unit_pos: &UnitPosition<RegionKey> = &unit_pos_str.parse().unwrap();
        let origin = unit_pos.region.clone();

        // Positions owned (in future this comes from API)
        let positions_owned: Vec<UnitPosition<RegionKey>> = vec![
            "FRA: F eng",
            "FRA: F nth",
        ]
        .iter()
        .map(|x| x.parse::<UnitPosition<RegionKey>>().unwrap())
        .collect();

        let positions: Vec<UnitPosition<&RegionKey>> = positions_owned
            .iter()
            .map(|up| UnitPosition::new(up.unit.clone(), &up.region))
            .collect();

        let mut adjacent_moves = Vec::<RegionKey>::new();
        let mut convoy_moves = Vec::<RegionKey>::new();

        // --- FIND ALL POSSIBLE DESTINATIONS ---
        for region in map.regions() {
            let dest = RegionKey::from_str(&region.short_name()).unwrap();

            if dest == origin {
                continue;
            }

            // ----- LAND MOVE CHECK -----
            let is_land_adjacent = map
                .find_border_between(&origin, &dest)
                .is_some();

            if is_land_adjacent {
                adjacent_moves.push(dest.clone());
                continue;
            }

            // Don't allow convoy to sea
            if region.terrain() == Terrain::Sea {
                continue;
            }

            if unit_pos.unit.unit_type() == UnitType::Army {
                // ----- CONVOY ROUTE CHECK -----
                let nation = Nation::from("FRA");
                let order = MappedMainOrder::new(
                    nation.clone(),
                    UnitType::Army,
                    origin.clone(),
                    order::MainCommand::Move(MoveCommand::new(dest.clone())),
                );

                if route_may_exist(map, positions.clone(), &order) {
                    convoy_moves.push(dest.clone());
                }
            }
        }

        // --- BUILD CHOICES ---
        let mut choice_strings = Vec::<String>::new();
        let mut choice_map = Vec::<(String, RegionKey)>::new();

        for dest in &adjacent_moves {
            let s = format!("{}", dest);
            choice_strings.push(s.clone());
            choice_map.push((s, dest.clone()));
        }

        for dest in &convoy_moves {
            // Do NOT include convoy if land is also available
            if adjacent_moves.contains(dest) {
                continue;
            }

            let s = format!("{} (convoy)", dest);
            choice_strings.push(s.clone());
            choice_map.push((s, dest.clone()));
        }

        if choice_strings.is_empty() {
            println!("No legal moves.");
            return Some(InputResult::Quit);
        }

        println!("Available destinations:");

        match Select::new("Choose destination:", choice_strings).prompt() {
            Ok(choice) => {
                let region = choice_map
                    .iter()
                    .find(|(s, _)| *s == choice)
                    .map(|(_, r)| r.clone())
                    .unwrap();

                machine_data.selected_destination = Some(region.to_string());
                println!("Selected move to {}", region);

                return Some(InputResult::Advance);
            }
            Err(_) => return Some(InputResult::Continue),
        }
    }

    fn next(self: Box<Self>, _sm: &mut StateMachine) -> Box<dyn State> {
        Box::new(ConfirmMove::new()) // <-- Your confirmation state
    }

    fn is_terminal(&self) -> bool {
        false
    }
}
