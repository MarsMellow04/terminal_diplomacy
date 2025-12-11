use diplomacy::{Unit,UnitPosition, UnitType};
use diplomacy::geo::RegionKey;
use diplomacy::order::{MainCommand, MoveCommand};

use crate::interactive::state_machine::{InputResult, MachineData, State};
use crate::interactive::state_machine::StateMachine;
use crate::interactive::states::show_units::ShowUnitState;
use crate::interactive::states::terminal_state::TerminalState;
use crate::rules::order_builder::OrderBuilder;

pub struct ConfirmHold {
}

impl ConfirmHold {
    pub fn new() -> Self {
        Self {}
    }
}

impl State for ConfirmHold {
    fn render(&self,  _machine: &StateMachine) {
        println!("\nConfirm this move command 'y', or 'q' to quit, 'b' to go back");
    }

    fn handle_input(&mut self, input: &str, _machine_data: &mut MachineData) -> Option<InputResult> {
        match input.trim() {
            "y" => return Some(InputResult::Advance),
            "q" => return Some(InputResult::Quit),
            "b" => return Some(InputResult::Continue), 
            _ => {return Some(InputResult::Continue)}
        }
    }

    fn next(self: Box<Self>, machine: &mut StateMachine) -> Box<dyn State> {
        let unit = machine.data.selected_unit.as_ref().unwrap().clone();

        // I want the parsing for orders to occur now
        let order = OrderBuilder::new()
            .nation("Fra")
            .for_unit(&unit)
            .hold()
            .build();
        machine.data.orders.push(order.unwrap());
        println!("{:?}", machine.data.orders);
        let index = machine.data.units_remaining.iter().position(|x| *x == unit).unwrap();
        let remaining_units = machine.data.units_remaining.remove(index);
        Box::new(ShowUnitState::new())
    }

    fn is_terminal(&self) -> bool {
        false
    }
}
