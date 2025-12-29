use std::{borrow::Cow, collections::{HashMap, HashSet}};

use diplomacy::{Nation, ShortName, Unit, UnitPosition, UnitType, geo::{Map, ProvinceKey, RegionKey, standard_map}, judge::MappedMainOrder};

#[derive(Clone)]
pub struct GameContext {
    pub user_nation: Nation,
    pub map: Map,
    last_owners: HashMap<ProvinceKey, Nation>, 
    occupiers: HashMap<ProvinceKey, Nation>,
    pub units: HashMap<Nation, HashSet<(UnitType, RegionKey)>>,
}

impl GameContext {
    pub fn new(
        user_nation: Nation, 
        last_owners: HashMap<ProvinceKey, Nation>, 
        occupiers: HashMap<ProvinceKey, Nation>,
        units: HashMap<Nation, HashSet<(UnitType, RegionKey)>>,
    ) -> Self {
        Self {
            user_nation: user_nation,
            map: standard_map().clone(),
            last_owners: last_owners,
            occupiers: occupiers,
            units: units,
        }
    }

    fn adapt_orders(&self, orders: Vec<MappedMainOrder>) -> HashSet<(UnitType, RegionKey)> {
        orders
            .iter()
            .fold(
                HashSet::default(), 
                |mut acc, ord| {
                    acc.insert((ord.unit_type, ord.region.clone()));
                    acc
                }
            )
    }

    pub fn find_player_units(&self) -> Option<&HashSet<(UnitType, RegionKey)>> {
        self.units.get(&self.user_nation)
    }

    pub fn remaining_units(&self, orders: &Vec<MappedMainOrder>) -> Option<HashSet<(UnitType, RegionKey)>> {
        let Some(player_units) = self.find_player_units() else {
            return None
        };
        player_units
            .difference(&self.adapt_orders(orders.to_vec()))
            .cloned()
            .collect::<HashSet<_>>()
            .into()
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