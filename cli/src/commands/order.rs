use crate::Command;

#[derive(Default)]
pub struct OrderCommand {
    name: String,
}

impl OrderCommand {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Command for OrderCommand {
    fn execute(&self) -> bool{
        false
    }
}