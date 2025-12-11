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


pub struct ConfirmConvoyMove {

}

impl ConfirmConvoyMove {
    pub fn new() -> Self {
        Self {  }
    }
}

impl State for ConfirmConvoyMove {
    fn render(&self, state_machine: &StateMachine) {}

    fn handle_input(&mut self, input: &str, machine_data: &mut MachineData) -> Option<InputResult> {
        // Check the adjacent units
        let from = machine_data.current_order.convoy_from.as_ref().unwrap();
        let to = machine_data.current_order.convoy_to.as_ref().unwrap();
        let fleet_unit = machine_data.selected_unit.as_ref().unwrap();

        let query_str = format!("Would you like to confirm this command:\n{} convoys {} -> {}",
            fleet_unit,
            from.short_name(),
            to.short_name()
        );
        
        let ans = Confirm::new(query_str.as_str())
            .with_default(false)
            .prompt();

        match ans {
            Ok(true) => {
                println!("That's awesome!");
                let order = machine_data.current_order.nation("FRA").for_unit(&fleet_unit).build().unwrap();
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