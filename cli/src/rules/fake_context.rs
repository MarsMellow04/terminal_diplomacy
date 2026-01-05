use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use diplomacy::{
    Nation,
    UnitType,
    geo::{RegionKey, ProvinceKey, standard_map},
};

use common::context::{GameContext, MapKind};

/// Temporary fake context until API exists
pub fn fake_game_context_france() -> GameContext {
    // --- User nation ---
    let user_nation = Nation::from("FRA");

    // --- Units ---
    let mut units: HashMap<Nation, HashSet<(UnitType, RegionKey)>> = HashMap::new();

    // France units
    units.insert(
        user_nation.clone(),
        HashSet::from([
            // (UnitType::Army, RegionKey::from_str("par").unwrap()),
            // (UnitType::Army, RegionKey::from_str("bur").unwrap()),
            // (UnitType::Army, RegionKey::from_str("pic").unwrap()),
            // (UnitType::Fleet, RegionKey::from_str("bre").unwrap()),
            // (UnitType::Army, RegionKey::from_str("pic").unwrap()),
            // (UnitType::Fleet, RegionKey::from_str("mao").unwrap()),
            // (UnitType::Army, RegionKey::from_str("spa(nc)").unwrap()),
            (UnitType::Fleet, RegionKey::from_str("bre").unwrap()),
            (UnitType::Army, RegionKey::from_str("mar").unwrap()),
            (UnitType::Army, RegionKey::from_str("par").unwrap()),
        ]),
    );

    // England (Channel / North Sea)
    let england = Nation::from("ENG");
    units.insert(
        england.clone(),
        HashSet::from([
            (UnitType::Fleet, RegionKey::from_str("eng").unwrap()),
            (UnitType::Fleet, RegionKey::from_str("nth").unwrap()),
        ]),
    );

    // Germany (border pressure)
    let germany = Nation::from("GER");
    units.insert(
        germany.clone(),
        HashSet::from([
            (UnitType::Army, RegionKey::from_str("mun").unwrap()),
            (UnitType::Army, RegionKey::from_str("ruh").unwrap()),
        ]),
    );

    // --- Ownership & occupation (explicit, minimal) ---
    let mut last_owners: HashMap<ProvinceKey, Nation> = HashMap::new();
    let mut occupiers: HashMap<ProvinceKey, Nation> = HashMap::new();

    // Helper: placing a unit implies occupation
    for (nation, unit_set) in &units {
        for (_, region) in unit_set {
            let province = region.province().clone();
            occupiers.insert(province.clone(), nation.clone());

            // If no prior owner known, assume owner = occupier
            last_owners
                .entry(province)
                .or_insert_with(|| nation.clone());
        }
    }

    GameContext::new(
        user_nation,
        MapKind::Standard,
        last_owners,
        occupiers,
        units,
    )
}
