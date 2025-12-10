use std::result;

use diplomacy::{UnitPosition, geo::{Map, RegionKey, standard_map}, judge::Submission};

use crate::interactive::{state_machine::{InputResult, State, StateMachine}, states::terminal_state::TerminalState};
use diplomacy::judge::MappedMainOrder;
use diplomacy::judge::Rulebook;
use diplomacy::judge::{OrderOutcome,AttackOutcome};


pub struct SelectSupportedUnitState {

}

impl SelectSupportedUnitState {
    pub fn new() -> Self {
        Self {}
    }
}

impl State for SelectSupportedUnitState {
    fn render(&self, state_machine: &StateMachine) {
        
    }

    fn handle_input(&mut self, input: &str, machine_data: &mut crate::interactive::state_machine::MachineData) -> Option<InputResult> {
        // Select a unit that is within range of the chosen destination
        let map = standard_map();
        if let Some(supported_region) = &machine_data.current_order.support_to {
            let other_units: Vec<UnitPosition<_>> = machine_data.all_units
                .iter()
                .filter(|&region| region != machine_data.selected_unit.as_ref().unwrap())
                .map(|string| string.parse::<UnitPosition<_>>().unwrap())
                .collect();

            let bordering_regions: Vec<&RegionKey> = map
                .find_bordering(supported_region);


            let possible_units: Vec<UnitPosition<_>>  = other_units
                .iter()
                .filter(|&unit_pos| {
                    // make move
                    let formatted = format!("{unit_pos} -> {supported_region}");
                    let order = formatted.parse::<MappedMainOrder>().unwrap();
                    let submission = Submission::with_inferred_state(map, vec![order]);
                    let outcome = submission.adjudicate(Rulebook::default());

                    let results: Vec<_> = outcome.all_orders_with_outcomes().collect();
                    let (_, result) = results[0];
                    *result == OrderOutcome::Move(AttackOutcome::Succeeds)
                })
                .cloned()
                .collect();

            let choices: Vec<String> = possible_units
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
                        println!("{:?} and {:?}", machine_data.current_order.support_from, machine_data.current_order.support_unit_type)
                    }
                    Err(_) => println!("Selection cancelled"),
                }
            } else {
                println!("No legal units.");
            };

        }
        None
    }

    fn next(self: Box<Self>, state_machine:&mut StateMachine) -> Box<dyn State> {
        Box::new(TerminalState)
    }

    fn is_terminal(&self) -> bool {
        false
    }
    
}