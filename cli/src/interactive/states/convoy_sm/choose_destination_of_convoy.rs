use diplomacy::geo::{Map, RegionKey, Terrain, standard_map};
use diplomacy::judge::{Adjudicate, ResolverState, Rulebook, Submission};
use diplomacy::judge::build::WorldState;
use diplomacy::judge::Context;
use diplomacy::order::{self, MainOrder, MoveCommand};
use diplomacy::{Nation, ShortName, Unit, UnitPosition, UnitType};
use serde_json::{json, map};
use diplomacy::judge::MappedMainOrder;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use diplomacy::geo::ProvinceKey;
use diplomacy;
use crate::interactive::states::convoy_sm::confirm_convoy::ConfirmConvoyMove;
use crate::interactive::states::convoy_sm::convoy::{route_may_exist};
use crate::interactive::{state_machine::{InputResult, MachineData, State, StateMachine}, states::terminal_state::TerminalState};


#[derive(Clone, PartialEq)]
pub struct ChooseConvoyMove;
struct CurrentWorldState {

}

impl WorldState for CurrentWorldState {
    fn nations(&self) -> HashSet<&Nation> { HashSet::new() }
    fn occupier(&self, prov: &ProvinceKey) -> Option<&Nation> { None }
    fn unit_count(&self, nat: &Nation) -> u8 { 0 }
    fn units(&self, nat: &Nation) -> HashSet<(UnitType, RegionKey)> { HashSet::new() }
}


impl State for ChooseConvoyMove {
    fn render(&self, state_machine: &StateMachine) {}

    fn handle_input(&mut self, input: &str, machine_data: &mut MachineData) -> Option<InputResult> {
        // Check the adjacent units
        if let Some(army_origin) = machine_data.current_order.convoy_from.as_ref() {
            let map = standard_map();
            let mut moves = vec![];
            
            // This will be an api call in the future
            let positions_owned: Vec<UnitPosition<RegionKey>> = vec![
                "FRA: F eng",
                "FRA: F nth",]
                .iter()
                .map(|&str| str.parse::<UnitPosition<RegionKey>>()
                .unwrap())
                .collect();
            
            let positions: Vec<UnitPosition<&RegionKey>> = positions_owned
                .iter()
                .map(|up| UnitPosition::new(up.unit.clone(), &up.region))
                .collect();

            // Check if convoy possible 
            for region in map.regions() {
                let region_key = &RegionKey::from_str(&region.short_name()).unwrap();
                if region_key == army_origin {
                    continue 
                }
                if region.terrain() == Terrain::Sea {
                    continue;
                }

                // Could theoretically get there 
                // Could theoretically get there 
                // Create fake move 
                let nation = Nation::from("FRA");
                let order = MappedMainOrder::new(nation.clone(), UnitType::Army, army_origin.clone(), order::MainCommand::Move(MoveCommand::new(region_key.clone())));
                
                if route_may_exist(map, positions.clone(), &order) {
                    moves.push(region_key.clone());
                }

            }
            moves.iter().for_each(|region| println!("THESE ARE THE PATHS!!!{:?}", region));

            // We now have the paths we just have to showcase to the user the options 
            let choices: Vec<String> = moves
                .iter()
                .map(|unit_pos| unit_pos.to_string())
                .collect();

            if !choices.is_empty() {
                use inquire::Select;

                println!("Available Destination :");

                match Select::new("Choose destination:", choices).prompt() {
                    Ok(choice) => {
                        println!("Selected: {}", choice);
                        machine_data.current_order.convoy_unit_to(&choice);
                        println!("{:?}", machine_data.current_order.convoy_to);
                        return(Some(InputResult::Advance))
                    }
                    Err(_) => return(Some(InputResult::Continue)),
                }
            } else {
                println!("No legal moves.");
            };
        }
        return Some(InputResult::Quit) 
    }

    fn next(self: Box<Self>, state_machine:&mut StateMachine) -> Box<dyn State> {
        Box::new(ConfirmConvoyMove::new())
    }

    fn is_terminal(&self) -> bool {
        false
    }
}