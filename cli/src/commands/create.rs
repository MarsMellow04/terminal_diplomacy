use crate::auth::session::SessionKeeper;
use crate::commands::util::{Command, Client, CommandError};

#[derive(Default)]
pub struct CreateCommand<C: Client, S: SessionKeeper> {
    pub client: C,
    session: S
}

impl <C: Client, S: SessionKeeper> CreateCommand<C, S> {
    pub fn new(client: C, session: S) -> Self {
        Self { client, session}
    }
}

impl <C: Client, S: SessionKeeper> CreateCommand<C, S> {
    pub fn execute(&mut self) -> Result<(), CommandError>{
        let session_token = self
            .session
            .load()
            .ok_or(CommandError::NoSessionToken)?;
        // CREATE;session_id\n
        let msg = format!("CREATE;{session_token}\n");
        self.client.send(&msg)
    }
}