use crate::interactive::state_machine::MachineData;
use crate::interactive::state_machine::State;
use crate::interactive::state_machine::StateMachine;
use crate::interactive::states::show_orders::ShowOrders;
use crate::interactive::states::terminal_state::TerminalState;
use crate::interactive::state_machine::InputResult;

pub struct ShowUnitState {
    pub current_units: Vec<String>,
    pub selected_unit: Option<String>,
}

impl ShowUnitState {
    pub fn new(units: Vec<String>) -> Self {
        Self {
            current_units: units,
            selected_unit: None,
        }
    }
}

impl State for ShowUnitState {
    fn render(&self, _machine: &StateMachine) {
        println!("\nPick a unit or 'q' to quit, 'b' to go back:");
        for (i, unit) in self.current_units.iter().enumerate() {
            println!("{} ) {}", i + 1, unit);
        }
    }

    fn handle_input(&mut self, input: &str, _machine_data: &mut MachineData) -> Option<InputResult> {
        match input.trim() {
            "q" => return Some(InputResult::Quit),
            "b" => return Some(InputResult::Continue), // update() will handle back
            _ => {}
        }

        let index = match input.parse::<usize>() {
            Ok(n) if n > 0 && n <= self.current_units.len() => n - 1,
            _ => return Some(InputResult::Continue),
        };

        self.selected_unit = Some(self.current_units[index].clone());
        Some(InputResult::Advance)
    }

    fn next(self: Box<Self>, machine: &mut StateMachine) -> Box<dyn State> {
        machine.data.selected_unit = self.selected_unit.clone();
        Box::new(ShowOrders::new(self.selected_unit.unwrap()))
    }

    fn is_terminal(&self) -> bool { false }
}
