use std::{collections::{HashMap, HashSet}, default};

use uuid::Uuid;
type UserId = Uuid;
use diplomacy::{Nation, Phase, UnitPosition, UnitType, geo::{Map, ProvinceKey, RegionKey, standard_map}};


pub struct GameInstance {
    idk_yet: String,
    pub players: HashMap<UserId, Nation>,
    pub phase: Phase,
    map: Map,
    last_owners: HashMap<ProvinceKey, Nation>, 
    occupiers: HashMap<ProvinceKey, Nation>,
    units: HashMap<Nation, HashSet<(UnitType, RegionKey)>>,
}

impl GameInstance {
    /// This is all far to overcompliciated but I am trying to figure out how lifetimes work
    /// I have now removed it anyway
    pub fn new() -> Self {
        Self {
            idk_yet: String::from("Hello"),
            players: HashMap::with_capacity(7),
            phase: Phase::Main,
            map: standard_map().clone(),
            last_owners: Default::default(),
            occupiers: Default::default(),
            units: Default::default(),
        }
    }

    pub fn is_full(&self) -> bool {
        // The maximum amount of players in Diplomacy is 7
        self.players.len() >= 7 
    }

    pub fn find_player_units(&self, user_id: &UserId) -> HashSet<(UnitType, RegionKey)> {
        self.players
            .get(user_id)
            // I think this line is very cool
            .and_then(|nation| self.units.get(nation))
            .cloned()
            .unwrap_or_default()
    }

    pub fn unit_count(&self, user_id: &UserId) -> u8 {
        self.players
            .get(user_id)
            .and_then(|nation| self.units.get(nation))
            .map(|u| u.len())
            .unwrap_or_default()
            .try_into()
            .unwrap()
    }


}