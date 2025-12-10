use diplomacy::{ShortName, UnitPosition, geo::{Map, RegionKey, Terrain}};
use diplomacy::geo::standard_map;

pub trait UnitStrategy {
    fn is_neight(
        &self,
        unit: &UnitPosition<'static, RegionKey>
    ) -> Vec<RegionKey>;
}