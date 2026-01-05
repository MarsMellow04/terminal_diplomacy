use async_trait::async_trait;
use uuid::Uuid;

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
        // This does a quick sanity check that the one recieved is the same:
        let token_str = self.client.read().await?;

        println!("[DEBUG] Recieved from the server: {}", token_str);

        let rec_token =
            Uuid::parse_str(&token_str).map_err(|_| CommandError::NoSessionToken)?;
        
        println!("[DEBUG] Have read from the server: {}", rec_token);

        let expec_token = self.session.load().ok_or(CommandError::NoSessionToken)?;
        assert_eq!(rec_token, expec_token);

        Ok(())
    }
}
