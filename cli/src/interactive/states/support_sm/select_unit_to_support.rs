use crate::interactive::state_machine::{InputResult, MachineData, State, StateMachine};
use crate::interactive::states::show_orders::SupportType;
use crate::interactive::states::support_sm::confirm_support::ConfirmSupport;
use crate::interactive::states::support_sm::select_supported_unit::SelectSupportedUnitState;
use crate::interactive::states::terminal_state::TerminalState;
use crate::interactive::states::support_sm::choose_support_dest::ChooseSupportUnitState;

use diplomacy::UnitPosition;
use diplomacy::geo::standard_map;
use inquire::{InquireError, Select};

pub struct SelectHoldToSupport {
}

impl SelectHoldToSupport {
    pub fn new() -> Self {
        Self { }
    }
}

impl State for SelectHoldToSupport {
    fn render(&self, state_machine: &StateMachine) {}
    fn handle_input(&mut self, input: &str, machine_data: &mut MachineData) -> Option<InputResult> {
        let possible_units: Vec<UnitPosition<_>> = machine_data.all_units.clone()
            .into_iter()
            .map(|str| str.parse::<UnitPosition<_>>().unwrap())
            .collect();

        // Posible units, find the adjacent units to it
        let unit_pos = machine_data.selected_unit.as_ref()
            .unwrap()
            .parse::<UnitPosition<_>>()
            .unwrap();

        let bordering = standard_map().find_bordering(&unit_pos.region);

        let adjacent_units: Vec<_> = possible_units
            .into_iter()
            .filter(|unit_pos| {
                bordering.contains(&&unit_pos.region)
            })
            .collect();

        let choices: Vec<String> = adjacent_units
                .iter()
                .map(|unit_pos| unit_pos.to_string())
                .collect();
        
        if !choices.is_empty() {
            use inquire::Select;

            println!("Available Units for that Support:");

            match Select::new("Choose a unit:", choices).prompt() {
                Ok(choice) => {
                    println!("Selected: {}", choice);
                    machine_data.current_order.support_unit(&choice);
                    println!("{:?} and {:?}", machine_data.current_order.support_from, machine_data.current_order.support_unit_type);
                    return Some(InputResult::Advance);
                }
                Err(_) => println!("Selection cancelled"),
            }
        } else {
            println!("No legal units.");
        };
        return Some(InputResult::Quit);
    }
    fn next(self: Box<Self>, state_machine:&mut StateMachine) -> Box<dyn State> {
        Box::new(ConfirmSupport::new())        
    }
    fn is_terminal(&self) -> bool {
        false
    }
}