use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::borrow::Cow;

use uuid::Uuid;
use diplomacy::{
    Nation, Phase, Unit, UnitPosition, UnitType,
    geo::{Map, ProvinceKey, RegionKey, standard_map},
};
use common::context::{GameContext, MapKind};

type UserId = Uuid;

fn get_starting_positions() -> HashMap<Nation, HashSet<(UnitType, RegionKey)>> {
    let data: &[(&str, &[(&str, &str)])] = &[
        ("aus", &[("Army","bud"),("Fleet","tri"),("Army","vie")]),
        ("eng", &[("Fleet","edi"),("Army","lvp"),("Fleet","lon")]),
        ("fra", &[("Fleet","bre"),("Army","mar"),("Army","par")]),
        ("ger", &[("Army","ber"),("Fleet","kie"),("Army","mun")]),
        ("ita", &[("Fleet","nap"),("Army","rom"),("Army","ven")]),
        ("rus", &[("Army","mos"),("Fleet","sev"),("Fleet","stp(sc)"),("Army","war")]),
        ("tur", &[("Fleet","ank"),("Army","con"),("Army","smy")]),
    ];

    let mut map = HashMap::new();
    for (nat, units) in data {
        let nation = Nation::from(*nat);
        map.insert(
            nation,
            units.iter()
                .map(|(t,r)| (
                    UnitType::from_str(t).unwrap(),
                    RegionKey::from_str(r).unwrap()
                ))
                .collect(),
        );
    }
    map
}

// Stupid crap i need to stop lifetime issues

#[derive(Debug, Clone)]
pub struct PendingRetreat {
    pub nation: Nation,
    pub unit_type: UnitType,
    pub from: RegionKey,
    pub options: HashSet<RegionKey>,
}


#[derive(Debug, Clone)]
pub struct GameInstance {
    pub players: HashMap<UserId, Nation>,
    pub phase: Phase,

    map: Map,
    pub last_owners: HashMap<ProvinceKey, Nation>,
    pub occupiers: HashMap<ProvinceKey, Nation>,
    pub units: HashMap<Nation, HashSet<(UnitType, RegionKey)>>,

    pub pending_retreats: Vec<PendingRetreat>,
}

impl GameInstance {
    pub fn new() -> Self {
        Self {
            players: HashMap::with_capacity(7),
            phase: Phase::Main,
            map: standard_map().clone(),
            last_owners: HashMap::new(),
            occupiers: HashMap::new(),
            units: get_starting_positions(),
            pending_retreats: Vec::new(),
        }
    }

    pub fn is_full(&self) -> bool {
        self.players.len() >= 7
    }

    pub fn map_used(&self) -> &Map {
        &self.map
    }

    pub fn apply_new_positions<I>(&mut self, positions: I)
    where
        I: IntoIterator<Item = UnitPosition<'static, RegionKey>>,
    {
        self.units.clear();
        self.occupiers.clear();

        for pos in positions {
            let nation = pos.unit.nation().clone();
            let ut = pos.unit.unit_type();
            let region = pos.region.clone();
            let province: ProvinceKey = region.province().clone();

            self.units.entry(nation.clone()).or_default().insert((ut, region));
            self.occupiers.insert(province, nation);
        }

        for prov in self.map.provinces().filter(|p| p.is_supply_center()) {
            let key: ProvinceKey = prov.into();
            if let Some(n) = self.occupiers.get(&key) {
                self.last_owners.insert(key, n.clone());
            }
        }
    }

    pub fn to_context_for(&self, user: &UserId) -> Option<GameContext> {
        let nation = self.players.get(user)?.clone();
        Some(GameContext::new(
            nation,
            MapKind::Standard,
            self.last_owners.clone(),
            self.occupiers.clone(),
            self.units.clone(),
        ))
    }
}

// Build phase support

use diplomacy::judge::build::WorldState;

impl WorldState for GameInstance {
    fn nations(&self) -> HashSet<&Nation> {
        self.units.keys().collect()
    }

    fn occupier(&self, province: &ProvinceKey) -> Option<&Nation> {
        self.occupiers.get(province)
    }

    fn unit_count(&self, nation: &Nation) -> u8 {
        self.units.get(nation).map(|u| u.len() as u8).unwrap_or(0)
    }

    fn units(&self, nation: &Nation) -> HashSet<(UnitType, RegionKey)> {
        self.units.get(nation).cloned().unwrap_or_default()
    }
}

impl GameInstance {
    pub fn find_player_units(
        &self,
        user_id: &Uuid,
    ) -> HashSet<(UnitType, RegionKey)> {
        let nation = match self.players.get(user_id) {
            Some(n) => n,
            None => return HashSet::new(),
        };

        self.units
            .get(nation)
            .cloned()
            .unwrap_or_default()
    }
}
