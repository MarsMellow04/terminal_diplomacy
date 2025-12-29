use crate::auth::session::SessionKeeper;
use crate::commands::util::{Client, Command, CommandError};
use uuid::Uuid;

#[derive(Default)]
pub struct RegisterCommand <C: Client, S: SessionKeeper> {
    client: C,
    session: S,
    username: String,
    password: String,
}

impl <C: Client, S: SessionKeeper> RegisterCommand<C,S>{
    pub fn new(client: C, session: S, username: String, password: String) -> Self {
        Self { client, session, username, password }
    }
}

impl <C: Client, S: SessionKeeper> RegisterCommand<C,S>{
    pub fn execute(&mut self) -> Result<(), CommandError>{
        // REGISTER;<username>;<password>\n
        let msg = format!("REGISTER;{};{}\n", self.username,self.password);
        self.client.send(&msg)?;
        let session_token = Uuid::parse_str(&self.client.read()?)
            .or(Err(CommandError::NoSessionToken))?;

        self.session.save(&session_token)
            .or(Err(CommandError::SessionSaveFailed))?;
        Ok(())
    }
}