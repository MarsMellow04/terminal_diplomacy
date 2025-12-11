use diplomacy::geo::{RegionKey, standard_map};
use diplomacy::{Unit, UnitPosition};

use crate::interactive::states::convoy_sm::choose_destination_of_convoy::ChooseConvoyMove;
use crate::interactive::{state_machine::{InputResult, MachineData, State, StateMachine}, states::terminal_state::TerminalState};



pub struct ChooseConvoyUnit {

}

impl ChooseConvoyUnit {
    pub fn new() -> Self {
        Self {  }
    }
}

impl State for ChooseConvoyUnit {
    fn render(&self, state_machine: &StateMachine) {}

    fn handle_input(&mut self, input: &str, machine_data: &mut MachineData) -> Option<InputResult> {
        // Check the adjacent units
        if let Some(fleet_str) = machine_data.selected_unit.as_ref() {
            let map = standard_map();
            let fleet_pos = fleet_str.parse::<UnitPosition<_>>().unwrap();
            let fleet_region = fleet_pos.as_region_ref().region;
            let bordering = map.find_bordering(fleet_region);

            let other_units: Vec<UnitPosition<_>>= machine_data.all_units.clone()
                .into_iter()
                .filter(|region| region != machine_data.selected_unit.as_ref().unwrap())
                .map(|string| string.parse::<UnitPosition<_>>().unwrap())
                .collect();
            
            let other_units: Vec<UnitPosition<'_, RegionKey>>= other_units
                .iter()
                .filter( |unit_position| {
                    bordering.contains(&&unit_position.region)
                })
                .cloned()
                .collect();

            let choices: Vec<String> = other_units
                .iter()
                .map(|unit_pos| unit_pos.to_string())
                .collect();

            if !choices.is_empty() {
                use inquire::Select;

                println!("Available Units for Convoy :");

                match Select::new("Choose a unit to convoy:", choices).prompt() {
                    Ok(choice) => {
                        println!("Selected: {}", choice);
                        machine_data.current_order.convoy_unit(&choice);
                        println!("{:?}", machine_data.current_order.convoy_from);
                        return(Some(InputResult::Advance))
                    }
                    Err(_) => return(Some(InputResult::Continue)),
                }
            } else {
                println!("No legal units.");
            };
        }
        return Some(InputResult::Quit)
    }

    fn next(self: Box<Self>, state_machine:&mut StateMachine) -> Box<dyn State> {
        Box::new(ChooseConvoyMove::new())
    }

    fn is_terminal(&self) -> bool {
        false
    }

     
}