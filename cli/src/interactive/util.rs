use core::fmt;
use std::fmt::Display;

use diplomacy::{Unit, UnitPosition, UnitType, geo::RegionKey};

use crate::{interactive::state_machine::{MachineData, StateMachine}, rules::{order_builder::OrderBuilder, strategies::order_strategy::OrderStrategy}};

pub enum SelectResult<T> {
    Selected(T),
    Back,
    Quit,
}

pub fn select_from<T: Clone + Display>(
    prompt: &str,
    items: &[T],
) -> SelectResult<T> {
    use inquire::Select;

    let mut choices: Vec<String> = items.iter().map(|i| i.to_string()).collect();
    let back_str = "<Back>";
    let quit_str = "Quit";
    
    choices.push(back_str.into());
    choices.push(quit_str.into());

    Select::new(prompt, choices)
        .prompt()
        .map(|choice| {
            if choice == back_str {
                SelectResult::Back
            } else if choice == quit_str {
                SelectResult::Quit
            } else {
                items
                    .iter()
                    .find(|i| i.to_string() == choice)
                    .cloned()
                    .map(SelectResult::Selected)
                    .unwrap_or(SelectResult::Back)
            }
        })
        .unwrap_or(SelectResult::Quit)
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct UnitAt(pub UnitType, pub RegionKey);

impl fmt::Display for UnitAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let UnitAt(unit_type, region) = self;
        write!(f, "{:?} {}", unit_type, region)
    }
}

impl From<(UnitType, RegionKey)> for UnitAt {
    fn from(value: (UnitType, RegionKey)) -> Self {
        UnitAt(value.0, value.1)
    }
}

impl From<UnitAt> for (UnitType, RegionKey) {
    fn from(unit: UnitAt) -> Self {
        (unit.0, unit.1)
    }
}

pub fn finalize_order(machine_data: &mut MachineData) {
    let unit = machine_data.selected_unit.take().unwrap();
    let intent = machine_data.order_intent.take().unwrap();

    let mut builder = machine_data.current_builder.clone();

    intent.apply(&unit, &mut builder);

    let order = builder.build().expect("validated earlier");
    machine_data.orders.push(order);
}

