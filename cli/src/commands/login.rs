use uuid::Uuid;

use crate::{auth::session::SessionKeeper, commands::util::{Client, Command, CommandError}};

#[derive(Default)]
pub struct LoginCommand <C: Client, S: SessionKeeper>{
    pub client: C,
    session: S,
    username: String,
    password: String,
}

impl <C: Client, S: SessionKeeper> LoginCommand<C,S>{
    pub fn new(client: C, session: S, username: String, password: String) -> Self {
        Self { client, session, username, password }
    }
}

impl <C: Client, S: SessionKeeper> LoginCommand<C,S>{
    pub fn execute(&mut self) -> Result<(), CommandError>{
        // LOGIN;<username>;<password>\n
        let msg = format!("LOGIN;{};{}\n", self.username,self.password);
        self.client.send(&msg)?;
        let session_token = Uuid::parse_str(&self.client.read()?)
            .or(Err(CommandError::NoSessionToken))?;

        self.session.save(&session_token)
            .or(Err(CommandError::SessionSaveFailed))?;
        Ok(())
    }
}