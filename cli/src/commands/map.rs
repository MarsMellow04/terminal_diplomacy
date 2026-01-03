use async_trait::async_trait;

use crate::{
    auth::session::SessionKeeper,
    commands::util::{Client, Command, CommandError},
};

pub struct MapCommand<C: Client, S: SessionKeeper> {
    client: C,
    session: S,
    save_image: bool,
}

impl<C: Client, S: SessionKeeper> MapCommand<C, S> {
    pub fn new(client: C, session: S, save_image: bool) -> Self {
        Self {
            client,
            session,
            save_image,
        }
    }
}

#[async_trait]
impl<C, S> Command for MapCommand<C, S>
where
    C: Client + Send,
    S: SessionKeeper + Send,
{
    async fn execute(&mut self) -> Result<(), CommandError> {
        // Stub for now — map API not implemented yet
        Err(CommandError::ConnectionFailure)
    }
}
