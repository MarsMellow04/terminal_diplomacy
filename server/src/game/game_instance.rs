use uuid::Uuid;
type UserId = Uuid;

pub struct GameInstance {
    idk_yet: String,
    pub players: Vec<UserId>
}
impl GameInstance {
    /// This is all far to overcompliciated but I am trying to figure out how lifetimes work
    /// I have now removed it anyway
    pub fn new() -> Self {
        Self {
            idk_yet: String::from("Hello"),
            players: Vec::new(),
        }
    }

    pub fn is_full(&self) -> bool {
        // The maximum amount of players in Diplomacy is 7
        self.players.len() >= 7 
    }
    


}