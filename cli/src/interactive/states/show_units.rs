use crate::interactive::state_machine::MachineData;
use crate::interactive::state_machine::State;
use crate::interactive::state_machine::StateMachine;
use crate::interactive::states::show_orders::ShowOrders;
use crate::interactive::states::terminal_state::TerminalState;
use crate::interactive::state_machine::InputResult;

pub struct ShowUnitState {
    pub selected_unit: Option<String>,
}

impl ShowUnitState {
    pub fn new() -> Self {
        Self {
            selected_unit: None,
        }
    }
}

impl State for ShowUnitState {
    fn render(&self, machine: &StateMachine) {
        println!("\nPick a unit or 'q' to quit, 'b' to go back:");
        for (i, unit) in machine.data.units_remaining.iter().enumerate() {
            println!("{} ) {}", i + 1, unit);
        }
    }

    fn handle_input(&mut self, input: &str, machine_data: &mut MachineData) -> Option<InputResult> {
        // If completed go to the terminal and print all orders
        if machine_data.units_remaining.is_empty() {
            println!("Orders complete");
            return Some(InputResult::Quit);
        }
        match input.trim() {
            "q" => return Some(InputResult::Quit),
            "b" => return Some(InputResult::Continue), // update() will handle back
            _ => {}
        }

        let index = match input.parse::<usize>() {
            Ok(n) if n > 0 && n <= machine_data.units_remaining.len() => n - 1,
            _ => return Some(InputResult::Continue),
        };

        self.selected_unit = Some(machine_data.units_remaining[index].clone());
        Some(InputResult::Advance)
    }

    fn next(self: Box<Self>, machine: &mut StateMachine) -> Box<dyn State> {
        machine.data.selected_unit = self.selected_unit.clone();
        Box::new(ShowOrders::new(self.selected_unit.unwrap()))
    }

    fn is_terminal(&self) -> bool { false }
}
