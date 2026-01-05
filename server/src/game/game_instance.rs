use std::{borrow::Cow, collections::{HashMap, HashSet}, default, str::FromStr};

use uuid::Uuid;
type UserId = Uuid;
use diplomacy::{Nation, Phase, Unit, UnitPosition, UnitType, geo::{Map, ProvinceKey, RegionKey, standard_map}};

fn get_starting_positions() -> HashMap<Nation, HashSet<(UnitType, RegionKey)>> {
    let starting_positions = vec![
        (Nation::from("aus"), vec![
            (UnitType::Army, RegionKey::from_str("bud").unwrap()),
            (UnitType::Fleet, RegionKey::from_str("tri").unwrap()),
            (UnitType::Army, RegionKey::from_str("vie").unwrap()),
        ]),
        (Nation::from("eng"), vec![
            (UnitType::Fleet, RegionKey::from_str("edi").unwrap()),
            (UnitType::Army, RegionKey::from_str("lvp").unwrap()),
            (UnitType::Fleet, RegionKey::from_str("lon").unwrap()),
        ]),
        (Nation::from("fra"), vec![
            (UnitType::Fleet, RegionKey::from_str("bre").unwrap()),
            (UnitType::Army, RegionKey::from_str("mar").unwrap()),
            (UnitType::Army, RegionKey::from_str("par").unwrap()),
        ]),
        (Nation::from("ger"), vec![
            (UnitType::Army, RegionKey::from_str("ber").unwrap()),
            (UnitType::Fleet, RegionKey::from_str("kie").unwrap()),
            (UnitType::Army, RegionKey::from_str("mun").unwrap()),
        ]),
        (Nation::from("ita"), vec![
            (UnitType::Fleet, RegionKey::from_str("nap").unwrap()),
            (UnitType::Army, RegionKey::from_str("rom").unwrap()),
            (UnitType::Army, RegionKey::from_str("ven").unwrap()),
        ]),
        (Nation::from("rus"), vec![
            (UnitType::Army, RegionKey::from_str("mos").unwrap()),
            (UnitType::Fleet, RegionKey::from_str("sev").unwrap()),
            (UnitType::Fleet, RegionKey::from_str("stp_sc").unwrap()),
            (UnitType::Army, RegionKey::from_str("war").unwrap()),
        ]),
        (Nation::from("tur"), vec![
            (UnitType::Fleet, RegionKey::from_str("ank").unwrap()),
            (UnitType::Army, RegionKey::from_str("con").unwrap()),
            (UnitType::Army, RegionKey::from_str("smy").unwrap()),
        ]),
    ];
    starting_positions
        .into_iter()
        .fold(
            HashMap::default(),
            |mut map, (nation, units)| {
                map.insert(nation, units.into_iter().collect());
                map
            }
        )

}

#[derive(Debug)]
pub struct GameInstance {
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
            players: HashMap::with_capacity(7),
            phase: Phase::Main,
            map: standard_map().clone(),
            last_owners: Default::default(),
            occupiers: Default::default(),
            units: get_starting_positions(),
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

    pub fn map_used(&self) -> &Map {
        &self.map
    }

    pub fn get_unit_positions(&self) -> Vec<UnitPosition<'static, RegionKey>> {
        let mut out = Vec::new();

        for (nation, unit_set) in self.units.iter() {
            for (unit_type, region) in unit_set {
                let pos = UnitPosition {
                    unit: Unit::new(Cow::Owned(nation.clone()), unit_type.clone()),
                    region: region.clone(), // OWNED
                };

                out.push(pos);
            }
        }

        out
    }


}