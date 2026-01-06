use async_trait::async_trait;
use common::context::GameContext;
use diplomacy::judge::MappedMainOrder;
use uuid::Uuid;

use crate::auth::session::SessionKeeper;
use crate::commands::util::{Client, Command, CommandError};
use crate::interactive::states::show_units::ShowUnitState;
use crate::interactive::state_machine::{State, StateMachine, UiState};

pub struct OrderCommand<C: Client, S: SessionKeeper> {
    client: C,
    session: S,
    orders: Option<String>, // shortcut flag (still unused)
}

impl<C: Client, S: SessionKeeper> OrderCommand<C, S> {
    pub fn new(
        client: C,
        session: S,
        orders: Option<String>,
    ) -> Self {
        Self {
            client,
            session,
            orders
        }
    }

    /// Shortcut flags hook (intentionally no-op for now)
    fn parse_flags(&self) -> Result<Vec<MappedMainOrder>, CommandError> {
        if let Some(orders) = &self.orders {
            println!("[DEBUG] Shortcut flags detected ");
            // Try to parse as Vec<String> first
            let order_strings: Vec<String> = serde_json::from_str(orders)
                .map_err(|e| {
                    println!("[DEBUG] Failed to parse as string array: {}", e);
                    CommandError::CannotParseOrder(e)
                })?;

            // Parse each string into MappedMainOrder
            let parsed_orders: Result<Vec<MappedMainOrder>, _> = order_strings
                .iter()
                .map(|s| s.parse::<MappedMainOrder>().map_err(|_| CommandError::WriteFailure))
                .collect();
            
            return parsed_orders;
        } else {
            Err(CommandError::FlagNotFound)
        }
    }

    async fn get_context(&mut self, session_token: Uuid) -> Result<GameContext, CommandError> {
        let msg = format!("CONTEXT;{}\n", session_token);
        self.client.send(&msg).await?;
        let context_rec = self.client.read().await?;
        let context: GameContext = serde_json::from_str(&context_rec)
            .or(Err(CommandError::NoContextFound))?;
        return Ok(context);
    } 
}

#[async_trait]
impl<C, S> Command for OrderCommand<C, S>
where
    C: Client + Send,
    S: SessionKeeper + Send,
{
    async fn execute(&mut self) -> Result<(), CommandError> {
        let session_token = self
            .session
            .load()
            .ok_or(CommandError::NoSessionToken)?;

        // TODO: I think result isn't the best return var choice
        let main_orders: Vec<MappedMainOrder> = match self.parse_flags() {
            Ok(orders) => {orders}
            Err(_) => {
                // Falling back to interactive mode
                let context = self
                .get_context(session_token)
                .await?;

                // CONTEXT;<session_token>\n
                let mut machine = StateMachine::new(
                    UiState::ShowUnit(ShowUnitState),
                    context
                );
                while !machine.is_finished() {
                    machine.state.render(&machine.data);

                    // TODO: I can actually get rid of the input part here
                    let input = String::new();
                    machine.update(input.trim());
                }
                machine.data.orders
            }
        };

        // Orders found, sending over as Json
        let orders_json =
            serde_json::to_string(&main_orders)
                .map_err(|_| CommandError::WriteFailure)?;

        // ORDER;MAIN;<session_id>;<orders>\n
        let msg = format!(
            "ORDER;MAIN;{};{}\n",
            session_token,
            orders_json
        );

        println!("{}", msg);

        self.client.send(&msg).await?;
        Ok(())
    }
}
