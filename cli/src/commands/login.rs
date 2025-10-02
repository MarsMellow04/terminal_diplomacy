use crate::Command;

#[derive(Default)]
pub struct LoginCommand {
    username: String,
    password: String,
}

impl LoginCommand {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

impl Command for LoginCommand {
    fn execute(&self) -> bool{
        false
    }
}