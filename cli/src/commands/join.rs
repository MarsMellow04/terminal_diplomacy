use async_trait::async_trait;

use crate::{
    auth::session::SessionKeeper,
    commands::util::{Client, Command, CommandError},
};

pub struct JoinCommand<C: Client, S: SessionKeeper> {
    pub client: C,
    session: S,
    game: String,
}

impl<C: Client, S: SessionKeeper> JoinCommand<C, S> {
    pub fn new(client: C, session: S, game: String) -> Self {
        Self {
            client,
            session,
            game,
        }
    }
}

#[async_trait]
impl<C, S> Command for JoinCommand<C, S>
where
    C: Client + Send,
    S: SessionKeeper + Send,
{
    async fn execute(&mut self) -> Result<(), CommandError> {
        let session_token = self
            .session
            .load()
            .ok_or(CommandError::NoSessionToken)?;

        // JOIN;<session_id>;<game_id>\n
        let msg = format!("JOIN;{};{}\n", session_token, self.game);

        self.client.send(&msg).await?;
        Ok(())
    }
}
