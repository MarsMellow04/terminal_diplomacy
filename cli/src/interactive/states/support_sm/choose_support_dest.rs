use crate::interactive::state_machine::{InputResult, MachineData, State};
use crate::interactive::state_machine::StateMachine;
use crate::interactive::states::support_sm::select_supported_unit::SelectSupportedUnitState;
use crate::interactive::states::terminal_state::TerminalState;
use std::fmt::{self, Display, Formatter};
use diplomacy::UnitPosition;
use diplomacy::geo::{Map, standard_map, RegionKey, Location};
use diplomacy::judge::{Adjudicate, AttackOutcome, Context, MappedMainOrder, OrderOutcome, Outcome, Rulebook, Submission};
use diplomacy::order::{MainCommand, MoveCommand};
use diplomacy::*;

pub struct ChooseSupportUnitState {
}

impl ChooseSupportUnitState {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl State for ChooseSupportUnitState {
    fn render(&self, machine: &StateMachine) {
        println!("\nPick a region to support a move to (number), 'q' to quit, or 'b' to go back:\n");
    }

    fn handle_input(&mut self, input: &str, machine_data: &mut MachineData) -> Option<InputResult> {
        let trimmed = input.trim();

        let map = standard_map();

        if let Some(supporter) = machine_data.selected_unit.as_ref() {
            let supporter_unit = supporter.parse::<UnitPosition<_>>().unwrap();
            let supporter_region = supporter_unit.as_region_ref().region;

            let bordering = map.find_bordering(supporter_region);

            // Build hypothetical move orders
            let test_orders: Vec<MappedMainOrder> = bordering
                .iter()
                .map(|dst| {
                    let formatted = format!("{supporter} -> {}", dst.short_name());
                    formatted.parse::<MappedMainOrder>().unwrap()
                })
                .collect();

            // Adjudicate
            let submission = Submission::with_inferred_state(map, test_orders);
            let outcome = submission.adjudicate(Rulebook::default());

            // Filter legal moves
            let correct_places: Vec<&RegionKey> = outcome
                .all_orders_with_outcomes()
                .filter_map(|(order, out)| {
                    if let OrderOutcome::Move(result) = out {
                        if *result == AttackOutcome::Succeeds {
                            return order.move_dest();
                        }
                    }
                    None
                })
                .collect();

            // Convert to inquire choices
            let choices: Vec<String> = correct_places
                .iter()
                .map(|reg| reg.short_name().to_string())
                .collect();

            let result = if !choices.is_empty() {
                use inquire::Select;

                println!("Available Regions to Support:");

                match Select::new("Choose a destination:", choices).prompt() {
                    Ok(choice) => {
                        println!("Selected: {}", choice);
                        machine_data.current_order.support_move_to(&choice);
                        println!("{:?}", machine_data.current_order.support_to.as_ref());
                        Some(InputResult::Advance)
                    }
                    Err(_) => {
                        println!("Selection cancelled");
                        Some(InputResult::Quit)
                    }
                }
            } else {
                println!("No legal destinations.");
                Some(InputResult::Quit)
            };
            return result;
        }
        return Some(InputResult::Quit);
    }

    fn next(self: Box<Self>, machine: &mut StateMachine) -> Box<dyn State> {
        Box::new(SelectSupportedUnitState::new())
    }

    fn is_terminal(&self) -> bool {
        false
    }
}