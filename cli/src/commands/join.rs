use crate::Command;

#[derive(Default)]
pub struct JoinCommand {
    game: String,
}

impl JoinCommand {
    pub fn new(game: String) -> Self {
        Self { game }
    }
}

impl Command for JoinCommand {
    fn execute(&self) -> bool{
        false
    }
}