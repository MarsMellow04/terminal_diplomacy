use diplomacy::{ShortName, UnitPosition, geo::{Map, RegionKey, Terrain}};
use diplomacy::geo::standard_map;

pub trait MoveStrategy {
    fn legal_destinations(
        &self,
        unit: &UnitPosition<'static, RegionKey>
    ) -> Vec<RegionKey>;
}