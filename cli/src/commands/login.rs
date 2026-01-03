use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    auth::session::SessionKeeper,
    commands::util::{Client, Command, CommandError},
};

pub struct LoginCommand<C: Client, S: SessionKeeper> {
    pub client: C,
    session: S,
    username: String,
    password: String,
}

impl<C: Client, S: SessionKeeper> LoginCommand<C, S> {
    pub fn new(client: C, session: S, username: String, password: String) -> Self {
        Self {
            client,
            session,
            username,
            password,
        }
    }
}

#[async_trait]
impl<C, S> Command for LoginCommand<C, S>
where
    C: Client + Send,
    S: SessionKeeper + Send,
{
    async fn execute(&mut self) -> Result<(), CommandError> {
        // LOGIN;<username>;<password>\n
        let msg = format!("LOGIN;{};{}\n", self.username, self.password);

        self.client.send(&msg).await?;

        let token_str = self.client.read().await?;
        let session_token =
            Uuid::parse_str(&token_str).map_err(|_| CommandError::NoSessionToken)?;

        self.session
            .save(&session_token)
            .map_err(|_| CommandError::SessionSaveFailed)?;

        Ok(())
    }
}
