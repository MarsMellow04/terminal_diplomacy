use crate::interactive::state_machine::{InputResult, MachineData, State, StateMachine};
use crate::interactive::states::move_sm::confirm_move::ConfirmMove;
use crate::interactive::states::show_orders::ShowOrders;
use crate::interactive::states::terminal_state::TerminalState;
use crate::rules::strategies::move_stategy::{ArmyMoveStrategy, FleetMoveStrategy, MoveStrategy};

use diplomacy::{UnitType, UnitPosition};
use diplomacy::geo::RegionKey;

pub struct PickMoveState {
    pub current_dests: Vec<RegionKey>,
    pub selected_dest: Option<RegionKey>,
}

impl PickMoveState {
    pub fn new() -> Self {
        Self {
            current_dests: vec![],
            selected_dest: None,
        }
    }
}

impl State for PickMoveState {
    fn render(&self, machine: &StateMachine) {
        println!("\nPick a destination (number), 'q' to quit, or 'b' to go back:\n");
        println!("This is the machine selected unit: {:?}", &machine.data.selected_unit);
        let Some(unit_str) = &machine.data.selected_unit else {
            println!("⚠ No unit selected.");
            return;
        };

        let Ok(unit_position) = unit_str.parse::<UnitPosition<'_, RegionKey>>() else {
            println!("⚠ Could not parse selected unit: {}", unit_str);
            return;
        };

        let unit_type = unit_position.unit.unit_type();

        let possible_moves = match unit_type {
            UnitType::Army => ArmyMoveStrategy{}.legal_destinations(&unit_position),
            UnitType::Fleet => FleetMoveStrategy{}.legal_destinations(&unit_position),
        };

        for (i, dest) in possible_moves.iter().enumerate() {
            println!("{:>2}) {}", i + 1, dest);
        }
    }

    fn handle_input(&mut self, input: &str, machine_data: &mut MachineData) -> Option<InputResult> {
        let trimmed = input.trim();

        match trimmed {
            "q" => return Some(InputResult::Quit),
            "b" => return Some(InputResult::Continue), // state machine handles back
            _ => {}
        }

        let Some(unit_str ) = machine_data.selected_unit.as_ref() else {
            println!("⚠ No unit selected.");
            return Some(InputResult::Continue);
        };

        let Ok(unit_position) = unit_str.parse::<UnitPosition<'_, RegionKey>>() else {
            println!("⚠ Could not parse selected unit: {}", unit_str);
            return Some(InputResult::Continue);
        };

        let unit_type = unit_position.unit.unit_type();

        let possible_moves = match unit_type {
            UnitType::Army => ArmyMoveStrategy{}.legal_destinations(&unit_position),
            UnitType::Fleet => FleetMoveStrategy{}.legal_destinations(&unit_position),
        };

        let index = match input.parse::<usize>() {
            Ok(n) if n > 0 && n <= possible_moves.len() => n - 1,
            _ => return Some(InputResult::Continue),
        };

        println!("{}", possible_moves[index].clone());
        machine_data.selected_destination = Some(possible_moves[index].to_string());
        Some(InputResult::Advance)
    }

    fn next(self: Box<Self>, machine: &mut StateMachine) -> Box<dyn State> {
        Box::new(ConfirmMove::new())
    }

    fn is_terminal(&self) -> bool {
        false
    }
}