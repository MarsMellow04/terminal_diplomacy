use async_trait::async_trait;
use std::io::{self, Write};

use crate::auth::session::SessionKeeper;
use crate::commands::util::{Client, Command, CommandError};
use crate::interactive::states::show_units::ShowUnitState;
use crate::interactive::state_machine::{State, StateMachine, UiState};
use crate::rules::fake_context::fake_game_context_france;

pub struct OrderCommand<C: Client, S: SessionKeeper> {
    client: C,
    session: S,
    name: Option<String>, // shortcut flag (still unused)
    game_id: String,
}

impl<C: Client, S: SessionKeeper> OrderCommand<C, S> {
    pub fn new(
        client: C,
        session: S,
        name: Option<String>,
        game_id: String,
    ) -> Self {
        Self {
            client,
            session,
            name,
            game_id,
        }
    }

    /// Shortcut flags hook (intentionally no-op for now)
    fn parse_flags(&self) -> Option<()> {
        if self.name.is_some() {
            println!("Shortcut flags detected (not implemented yet)");
            Some(())
        } else {
            None
        }
    }
}

#[async_trait]
impl<C, S> Command for OrderCommand<C, S>
where
    C: Client + Send,
    S: SessionKeeper + Send,
{
    async fn execute(&mut self) -> Result<(), CommandError> {
        // 1️⃣ Shortcut path (future use)
        if self.parse_flags().is_some() {
            println!("Skipping interactive mode (not yet implemented)");
            // fall through to interactive for now
        }

        // 2️⃣ Start interactive FSM (blocking, intentionally)
        let mut machine = StateMachine::new(
            UiState::ShowUnit(ShowUnitState),
            fake_game_context_france(),
        );

        while !machine.is_finished() {
            machine.state.render(&machine.data);

            let mut input = String::new();
            machine.update(input.trim());
        }

        // 3️⃣ FSM finished → build & send order
        let session_token = self
            .session
            .load()
            .ok_or(CommandError::NoSessionToken)?;

        let orders_json =
            serde_json::to_string(&machine.data.orders)
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
