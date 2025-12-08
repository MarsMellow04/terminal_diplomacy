use diplomacy::{ShortName, UnitPosition, geo::{Map, RegionKey, Terrain}};
use diplomacy::geo::standard_map;
pub trait MoveStrategy {
    fn legal_destinations(
        &self,
        unit: &UnitPosition<'static, RegionKey>
    ) -> Vec<RegionKey>;
}

pub struct ArmyMoveStrategy {}

impl MoveStrategy for ArmyMoveStrategy {
    fn legal_destinations(
        &self,
        unit: &UnitPosition<'static, RegionKey>
    ) -> Vec<RegionKey> {
        let map: &Map = standard_map();
        let possible_terrains: [Terrain; 2] = [Terrain::Land, Terrain::Coast];

        let neighbors = map.find_bordering(&unit.region);

        neighbors
            .into_iter()
            .filter_map(|rk| {
                let region = map.find_region(&rk.short_name())?;
                if possible_terrains.contains(&region.terrain()) {
                    Some(rk.clone()) // &RegionKey → RegionKey
                } else {
                    None
                }
            })
            .collect()
    }
}

pub struct FleetMoveStrategy {}

impl MoveStrategy for FleetMoveStrategy {
    fn legal_destinations(
        &self,
        unit: &UnitPosition<'static, RegionKey>
    ) -> Vec<RegionKey> {
        let map: &Map = standard_map();
        let possible_terrains: [Terrain; 2] = [Terrain::Sea, Terrain::Coast];

        let neighbors = map.find_bordering(&unit.region);

        neighbors
            .into_iter()
            .filter_map(|rk| {
                let region = map.find_region(&rk.short_name())?;
                if possible_terrains.contains(&region.terrain()) {
                    Some(rk.clone()) // &RegionKey → RegionKey
                } else {
                    None
                }
            })
            .collect()
    }
}