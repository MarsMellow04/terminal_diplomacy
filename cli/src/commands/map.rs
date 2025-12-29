use crate::{auth::session::SessionKeeper, commands::util::{Client, Command, CommandError}};

#[derive(Default)]
pub struct MapCommand <C: Client, S: SessionKeeper> {
    client: C,
    session: S,
    save_image: bool
}

impl <C: Client, S: SessionKeeper> MapCommand<C,S> {
    pub fn new(client: C, session: S, save_image: bool) -> Self {
        Self { client, session, save_image }
    }
}

impl <C: Client, S: SessionKeeper> MapCommand<C,S> {
    pub fn execute(&self) -> Result<(), CommandError>{
        Err(CommandError::ConectionFailure)
    }
}