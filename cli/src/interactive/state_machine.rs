use core::fmt;
use std::default;

use diplomacy::{UnitPosition, UnitType, geo::RegionKey, judge::MappedMainOrder};
use crate::{interactive::states::{convoy_sm::{choose_destination_of_convoy::ChooseConvoyMove, choose_unit_to_convoy::ChooseConvoyUnit, confirm_convoy::ConfirmConvoyMove}, hold_sm::confirm_hold::ConfirmHold, move_sm::{confirm_move::ConfirmMove, pick_move::PickMoveState}, show_orders::ShowOrders, show_units::ShowUnitState, support_sm::{choose_support_dest::ChooseSupportUnitState, select_unit_to_support::SelectHoldToSupport}, terminal_state::TerminalState}, rules::{game_context::GameContext, order_builder::OrderBuilder}};


pub trait State {
    /// Render the prompt for this state
    fn render(&self, machine_data: &MachineData);
    fn handle_input(&mut self, input: &str, machine_data: &mut MachineData, ctx: &GameContext) -> InputResult;
    fn next(self, machine_data: &mut MachineData) -> UiState;
    fn is_terminal(&self) -> bool;
}

pub enum InputResult {
    Continue,
    Back,
    Advance,
    Quit,
}

struct StateSnapshot {
    pub data: MachineData,
    pub state: UiState,
}

#[derive(Debug, Clone)]
pub enum OrderIntent {
    Hold,
    Move { to: RegionKey },
    SupportHold { target: UnitPosition<'static, RegionKey> },
    SupportMove {
        target: UnitPosition<'static, RegionKey>,
        to: RegionKey,
    },
    Convoy {
        target: UnitPosition<'static, RegionKey>,
        to: RegionKey,
    },
}

#[derive(Debug, Default, Clone)]
pub struct OrderDraft {
    pub kind: Option<OrderKind>,
    pub move_to: Option<RegionKey>,
    pub target: Option<UnitPosition<'static, RegionKey>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderKind {
    Hold,
    Move,
    SupportHold,
    SupportMove,
    Convoy,
}

impl fmt::Display for OrderKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderKind::Hold => write!(f, "Hold"),
            OrderKind::Move => write!(f, "Move"),
            OrderKind::SupportMove => write!(f, "Support Move"),
            OrderKind::SupportHold => write!(f, "Support Hold"),
            OrderKind::Convoy => write!(f, "Convoy"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum UiState {
    ShowUnit(ShowUnitState),
    ShowOrder(ShowOrders),
    Terminal(TerminalState),
    ChooseConvoyDestination(ChooseConvoyMove),
    ConfimHold(ConfirmHold),
    ShowMoves(PickMoveState),
    SelectSupportedUnit(SelectHoldToSupport),
    SelectSupportedDestination(ChooseSupportUnitState),
    ConfirmMove(ConfirmMove),
    ConfirmConvoy(ConfirmConvoyMove),
    ChooseUnitToConvoy(ChooseConvoyUnit),
}

impl State for UiState {
    fn render(&self, machine_data: &MachineData) {
        match self {
            UiState::Terminal(s) => {s.render(machine_data);}
            UiState::ShowUnit(s) => {s.render(machine_data);}
            UiState::ShowOrder(s) => {s.render(machine_data);}
        }
    }
    fn handle_input(&mut self, input: &str, machine_data: &mut MachineData, ctx: &GameContext) -> InputResult {
        match self {
            UiState::Terminal(s) => {s.handle_input(input, machine_data, ctx)}
            UiState::ShowUnit(s) => {s.handle_input(input, machine_data, ctx)}
            UiState::ShowOrder(s) => {s.handle_input(input, machine_data, ctx)}
        }
    }
    fn next(self, machine_data: &mut MachineData) -> Self {
        match self {
            UiState::Terminal(s) => {s.next(machine_data)}
            UiState::ShowUnit(s) => {s.next(machine_data)}
            UiState::ShowOrder(s) => {s.next(machine_data)}
        }
    }
    fn is_terminal(&self) -> bool {
        match self {
            UiState::Terminal(s) => {true}
            _ => {false}
        }
    }
}

#[derive(Clone)]
pub struct MachineData {
    // All info needed by a state
    pub selected_unit: Option<UnitPosition<'static, RegionKey>>,
    pub selected_destination: Option<RegionKey>,
    pub order_intent: Option<OrderIntent>,
    pub order_draft: Option<OrderDraft>,
    pub orders: Vec<MappedMainOrder>,
    pub current_builder: OrderBuilder,
}


pub struct StateMachine {
    pub data: MachineData,
    pub state: UiState,
    pub history: Vec<StateSnapshot>,
    pub game_context: GameContext,
}

impl StateMachine {
    pub fn new(inital_state:UiState, game_context: GameContext) -> Self {
        Self {
            data: MachineData { 
                selected_unit: None, 
                selected_destination: None, 
                order_intent: None, 
                orders: Vec::new(), 
                current_builder: OrderBuilder::new(&game_context.user_nation),
                order_draft: None,
            },
            state: inital_state,
            history: Vec::new(),
            game_context:game_context
        }
    }

    pub fn update(&mut self, input: &str) {
        let input_result =  self
            .state
            .handle_input(input, &mut self.data, &self.game_context);
        
        match input_result {
            InputResult::Continue => {}
            InputResult::Quit => {
                self.state = UiState::Terminal(TerminalState);
            }
            InputResult::Advance => {
                self.history.push(StateSnapshot { data: self.data.clone(), state: self.state.clone() });
                // Set to new state
                self.state = self.state.clone().next(&mut self.data);
            }
            InputResult::Back => {
                if let Some(snapshot) = self.history.pop() {
                    self.state = snapshot.state;
                    self.data = snapshot.data;
                }
            }
        } 
    }

    pub fn is_finished(&self) -> bool {
        self.state.is_terminal()
    } 
}