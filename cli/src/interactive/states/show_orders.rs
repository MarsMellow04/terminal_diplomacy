use diplomacy::{UnitType};
use crate::interactive::state_machine::{InputResult, MachineData, OrderDraft, OrderKind, State, UiState};
use crate::interactive::states::convoy_sm::choose_destination_of_convoy::ChooseConvoyMove;
use crate::interactive::states::hold_sm::confirm_hold::ConfirmHold;
use crate::interactive::states::move_sm::pick_move::PickMoveState;
use crate::interactive::states::support_sm::choose_support_dest::ChooseSupportUnitState;
use crate::interactive::states::support_sm::select_unit_to_support::SelectHoldToSupport;
use crate::interactive::util::{SelectResult,select_from};

fn find_possible_orders(unit_type: UnitType) -> Vec<OrderKind>{
        match unit_type {
            UnitType::Army => {
                vec! [ 
                    OrderKind::Hold,
                    OrderKind::Move, 
                    OrderKind::SupportHold,
                    OrderKind::SupportMove,
                ]
            }
            UnitType::Fleet => {
                vec! [
                    OrderKind::Hold,
                    OrderKind::Move, 
                    OrderKind::SupportHold,
                    OrderKind::SupportMove,
                    OrderKind::Convoy,
                ]
            }
        }
    }

#[derive(Clone, PartialEq)]
pub struct ShowOrders;

impl State for ShowOrders {
    fn render(&self, data: &MachineData) {
        let unit_at = data.selected_unit.clone().unwrap();
        println!("\nSelected unit {:?}:", unit_at);
    }

    fn handle_input(&mut self, _input: &str, data: &mut MachineData, _ctx: &crate::rules::game_context::GameContext) -> InputResult {
        let unit = data.selected_unit.clone().unwrap();
        let unit_type = unit.unit.unit_type();

        let possible_orders = find_possible_orders(unit_type);
        match select_from("Select command: ", &possible_orders) {
            SelectResult::Selected(order_kind) => {
                data.order_draft = Some(OrderDraft { 
                    kind: Some(order_kind), 
                    move_to: None, 
                    support_target: None
                });
                InputResult::Advance
            }
            SelectResult::Back => {InputResult::Back}
            SelectResult::Quit => {InputResult::Quit}
        }
    }

    fn next(self, machine_data: &mut MachineData) -> crate::interactive::state_machine::UiState {
        match machine_data.order_draft.clone().unwrap().kind.unwrap() {
            OrderKind::Convoy => UiState::ShowConvoyDestination(ChooseConvoyMove),
            OrderKind::Hold => UiState::ConfimHold(ConfirmHold),
            OrderKind::Move => UiState::ShowMoves(PickMoveState),
            OrderKind::SupportHold => UiState::SelectSupportedUnit(SelectHoldToSupport),
            OrderKind::SupportMove => UiState::SelectSupportedDestination(ChooseSupportUnitState),
        }
    }

    fn is_terminal(&self) -> bool {
        false
    }
}
