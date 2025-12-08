use diplomacy::{Nation, Order, UnitPosition, UnitType, geo::RegionKey, judge::MappedMainOrder, order::{MainCommand, MoveCommand}};
use std::str::FromStr;

#[derive(Debug)]
pub enum OrderBuildError {
    UnknownLocation(String),
    UnkownUnitType(String),
    MissingUnitType,
    MissingOrigin,
    MissingCommand,
    IllegalSupportTarget,
}

pub struct OrderBuidler{
    command_type: Option<MainCommand<RegionKey>>,
    unit_position: Option<UnitPosition<'static, RegionKey>>,
    nation: Option<Nation>
}

impl OrderBuidler {
    pub fn new() -> Self {
        Self {
            command_type: None,
            unit_position: None,
            nation: None,
        }
    }

    pub fn nation(mut self, nation: &str) -> Self {
        // Parse the unit_position
        self.nation = Some(Nation::from(nation));
        self
    }

    pub fn for_unit(mut self, unit_position: &str) -> Self {
        // Parse the unit_position
        self.unit_position = match unit_position.parse() {
            Ok(pos) => {Some(pos)}
            Err(_e) => {
                return self;
                // return Err(OrderBuildError::UnkownUnitType(unit_position.to_string()));
            }
        };
        self
    }

    pub fn move_to(mut self, region: &str) -> Self {
        // Parse the unit_position
        let region = match RegionKey::from_str(region) {
            Ok(reg) => {reg}
            Err(_e) => {
                return self;
                // return Err(OrderBuildError::UnknownLocation(region.to_string()));
            }
        };
        self.command_type = Some(MainCommand::Move(MoveCommand::new(region)));
        self
    }

    pub fn hold(mut self ) -> Self {
        self.command_type = Some(MainCommand::Hold);
        self 
    }

    pub fn build(self) -> MappedMainOrder {
        // There could be a runtime error here
        MappedMainOrder::new_from_position(self.unit_position.unwrap(), self.command_type.unwrap())
    }
}
