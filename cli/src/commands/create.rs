use async_trait::async_trait;

use crate::{
    auth::session::SessionKeeper,
    commands::util::{Client, Command, CommandError},
};

pub struct CreateCommand<C: Client, S: SessionKeeper> {
    pub client: C,
    session: S,
}

impl<C: Client, S: SessionKeeper> CreateCommand<C, S> {
    pub fn new(client: C, session: S) -> Self {
        Self { client, session }
    }
}

#[async_trait]
impl<C, S> Command for CreateCommand<C, S>
where
    C: Client + Send,
    S: SessionKeeper + Send,
{
    async fn execute(&mut self) -> Result<(), CommandError> {
        let session_token = self
            .session
            .load()
            .ok_or(CommandError::NoSessionToken)?;

        // CREATE;<session_id>\n
        let msg = format!("CREATE;{}\n", session_token);

        self.client.send(&msg).await?;
        Ok(())
    }
}
