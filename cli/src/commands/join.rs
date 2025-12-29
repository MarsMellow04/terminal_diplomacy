use crate::commands::util::{Client, Command, CommandError};
use crate::auth::session::SessionKeeper;

#[derive(Default)]
pub struct JoinCommand<C: Client, S: SessionKeeper> {
    pub client: C,
    session: S,
    game: String
}

impl <C: Client, S: SessionKeeper> JoinCommand<C, S> {
    pub fn new(client: C, session: S, game: String) -> Self {
        Self { client, session, game}
    }
}

impl <C: Client, S: SessionKeeper > JoinCommand<C,S>{
    pub fn execute(&mut self) -> Result<(), CommandError>{
        let session_token = self
            .session
            .load()
            .ok_or(CommandError::NoSessionToken)?;

        // JOIN;GAME_ID;<session_id>;<join_id>\n
        let msg = format!("JOIN;{};{}\n",session_token, self.game);
        self.client.send(&msg)
    }
}