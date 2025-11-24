use crate::interactive::state_machine::State;
use crate::interactive::state_machine::StateMachine;
use crate::interactive::states::show_orders::ShowOrders;
use crate::interactive::states::terminal_state::TerminalState;

pub struct ShowUnitState {
    pub current_units: Vec<String>,
    pub selected_unit: Option<String>,
    pub wish_to_quit: bool,
    pub needs_restart: bool,
}

impl ShowUnitState {
    pub fn new(units: Vec<String>) -> Self {
        Self {
            current_units: units,
            selected_unit: None,
            wish_to_quit: false,
            needs_restart: false
        }
    }
}

impl State for ShowUnitState {
    fn render(&self) {
        // Get current units
        if self.needs_restart {
            println!("Please re-pick!");
        }
        else {
            println!("\nEnter a number or 'q' to quit:");
            println!("\n\n Pick a unit: \n\n");
            for (i, unit) in self.current_units.iter().enumerate() {
                println!("{} ) {}", i + 1, unit)
            }
        }
    }

    fn handle_input(&mut self, input: &str) {
        // Options for input: quit, select, if incorrect input next is same
        if input == "q" {
            self.wish_to_quit = true;
            println!("Ok thank you!");
            return ;

        }

        // Compare the input (parsing)
        let index = match input.trim().parse::<usize>() {
            Ok(num) if num > 0 => num - 1,
            _ => {
                self.needs_restart = true;
                return;
            }
        };

        // Check that the input is in bounds
        if index >= self.current_units.len() {
            self.needs_restart = true;
            return;
        }

        self.selected_unit = Some(self.current_units[index].clone());
    }

    fn next(self: Box<Self>, machine: &mut StateMachine) -> Box<dyn State> {
        if self.wish_to_quit {
            // Move to terminal
            return Box::new(TerminalState)
        }
        if self.selected_unit.is_none() {
            // Restart
            return Box::new(*self)
        }
        machine.data.selected_unit = self.selected_unit.clone();
        return Box::new(ShowOrders::new(self.selected_unit.unwrap()));
        
    }

    fn is_terminal(&self) -> bool {
        false
    }
}
