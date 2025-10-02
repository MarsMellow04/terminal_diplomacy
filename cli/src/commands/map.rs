use crate::Command;

#[derive(Default)]
pub struct MapCommand {
    save_image: bool
}

impl MapCommand {
    pub fn new(save_image: bool) -> Self {
        Self { save_image }
    }
}

impl Command for MapCommand {
    fn execute(&self) -> bool{
        false
    }
}