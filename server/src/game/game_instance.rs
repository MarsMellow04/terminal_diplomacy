pub struct GameInstance {
    idk_yet: String,
}
impl GameInstance {
    /// This is all far to overcompliciated but I am trying to figure out how lifetimes work
    /// I have now removed it anyway
    pub fn new() -> Self {
        Self {
            idk_yet: String::from("Hello")
        }
    }
}