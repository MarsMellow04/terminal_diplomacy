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
use crate::interactive::states::convoy_sm::convoy::{route_may_exist};
use crate::interactive::states::show_units::ShowUnitState;
use crate::interactive::{state_machine::{InputResult, MachineData, State, StateMachine}, states::terminal_state::TerminalState};
use crate::rules::order_builder;

use inquire::Confirm;


pub struct ConfirmSupport {

}

impl ConfirmSupport {
    pub fn new() -> Self {
        Self {  }
    }
}

impl State for ConfirmSupport {
    fn render(&self, state_machine: &StateMachine) {}

    fn handle_input(&mut self, input: &str, machine_data: &mut MachineData) -> Option<InputResult> {
        // Check the adjacent units
        println!("{:?}, {:?}, {:?}", 
            machine_data.selected_unit.as_ref(),
            machine_data.current_order.support_unit_type.as_ref(),
            machine_data.current_order.support_from.as_ref(),
        );
        let supporter_unit = machine_data.selected_unit.as_ref().expect("hello");
        let unit_type = machine_data.current_order.support_unit_type.as_ref().unwrap();
        let from = machine_data.current_order.support_from.as_ref().unwrap();
        let mut query_str = String::new();
        if let Some(support_move) = machine_data.current_order.support_to.as_ref() {
            // Branch for a support move
            query_str = format!("Would you like to confirm this command:\n{} supports {} {} -> {}",
                supporter_unit,
                unit_type.short_name(),
                from.short_name(),
                support_move.short_name()
            );
        } else {
            // Branch for support hold
            query_str = format!("Would you like to confirm this command:\n{} supports {} {} hold",
                supporter_unit,
                unit_type.short_name(),
                from.short_name(),
            );   
        }
        
        let ans = Confirm::new(query_str.as_str())
            .with_default(false)
            .prompt();

        match ans {
            Ok(true) => {
                println!("That's awesome!");
                let order = machine_data.current_order.nation("FRA").for_unit(&supporter_unit).build().unwrap();
                machine_data.orders.push(order.clone());
                println!("{:?}", order.clone());
                return Some(InputResult::Advance)
            }

            Ok(false) => {
                println!("That's too bad, I've heard great things about it.");
                return Some(InputResult::Quit)
            }
            Err(_) => {
                println!("Error with questionnaire, try again later");
                return Some(InputResult::Quit)
            }
        }
    }

    fn next(self: Box<Self>, state_machine:&mut StateMachine) -> Box<dyn State> {
        let index = state_machine.data.units_remaining.iter().position(|x| x == state_machine.data.selected_unit.as_ref().unwrap()).unwrap();
        let remaining_units = state_machine.data.units_remaining.remove(index);
        Box::new(ShowUnitState::new())
    }

    fn is_terminal(&self) -> bool {
        false
    }
}