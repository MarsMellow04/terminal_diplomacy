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
use crate::interactive::state_machine::UiState;
use crate::interactive::states::convoy_sm::convoy::{route_may_exist};
use crate::interactive::states::show_units::ShowUnitState;
use crate::interactive::{state_machine::{InputResult, MachineData, State, StateMachine}, states::terminal_state::TerminalState};
use crate::rules::order_builder;
use crate::interactive::util::{SelectResult, finalize_order, select_from};

use inquire::Confirm;

#[derive(Clone, PartialEq)]
pub struct ConfirmSupport;

impl State for ConfirmSupport {
    fn render(&self, _: &MachineData) {}

    fn handle_input(&mut self, _input: &str, machine_data: &mut MachineData, _ctx: &crate::rules::game_context::GameContext) -> InputResult {
        let options = vec!["Yes", "No"];
        match select_from("Do you confirm this order?", &options) {
            SelectResult::Selected("Yes") => {
                finalize_order(machine_data);
                InputResult::Advance
            }
            SelectResult::Selected(_) => {
                println!("Ok! Moving you back... ");
                InputResult::Back}

            SelectResult::Back => {InputResult::Back}
            SelectResult::Quit => {InputResult::Quit}
        }
    }

    fn next(&self, _machine_data: &mut MachineData) -> crate::interactive::state_machine::UiState {
        UiState::ShowUnit(ShowUnitState) 
    }

    fn is_terminal(&self) -> bool {
        false
    }
}