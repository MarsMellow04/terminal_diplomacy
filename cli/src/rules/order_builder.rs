use diplomacy::{
    Nation,
    UnitPosition,
    UnitType,
    geo::RegionKey,
    judge::MappedMainOrder,
    order::{MainCommand, MoveCommand, SupportedOrder},
};
use std::str::FromStr;

#[derive(Debug)]
pub enum OrderBuildError {
    MissingUnitPosition,
    MissingNation,
    MissingCommand,
    InvalidRegion(String),
    InvalidUnit(String),
}

#[derive(Debug, Clone)]
pub struct OrderBuilder {
    // Who owns the unit (optional, for completeness)
    nation: Option<Nation>,

    // The unit issuing the order
    unit_position: Option<UnitPosition<'static, RegionKey>>,

    // The command type (Hold, Move, Support)
    command: Option<MainCommand<RegionKey>>,

    // Support move info (optional until build() time)
    pub support_unit_type: Option<UnitType>,
    pub support_from: Option<RegionKey>,
    pub support_to: Option<RegionKey>,

    // Convoy Crap 
    pub convoy_from: Option<RegionKey>,
    pub convoy_to: Option<RegionKey>,
}

impl OrderBuilder {
    pub fn new() -> Self {
        Self {
            nation: None,
            unit_position: None,
            command: None,
            support_unit_type: None,
            support_from: None,
            support_to: None,
            convoy_from: None,
            convoy_to: None
        }
    }

    // -----------------------------------------------------------------
    // BASIC ORDER SETUP
    // -----------------------------------------------------------------

    pub fn nation(&mut self, nation: &str) -> &mut Self {
        self.nation = Some(Nation::from(nation));
        self
    }

    pub fn for_unit(&mut self, unit: &str) -> &mut Self {
        match unit.parse::<UnitPosition<RegionKey>>() {
            Ok(pos) => self.unit_position = Some(pos),
            Err(_) => eprintln!("Invalid unit position: {}", unit),
        }
        self
    }

    // -----------------------------------------------------------------
    // MOVEMENT & HOLD
    // -----------------------------------------------------------------

    pub fn hold(&mut self) -> &mut Self {
        self.command = Some(MainCommand::Hold);
        self
    }

    pub fn move_to(&mut self, region: &str) -> &mut Self {
        let region_key = match RegionKey::from_str(region) {
            Ok(r) => r,
            Err(_) => {
                eprintln!("Invalid region: {}", region);
                return self;
            }
        };

        self.command = Some(MainCommand::Move(MoveCommand::new(region_key)));
        self
    }

    // -----------------------------------------------------------------
    // SUPPORT ORDERS
    // -----------------------------------------------------------------

    /// Specify which unit is being supported
    pub fn support_unit(&mut self, unit: &str) -> &mut Self {
        match unit.parse::<UnitPosition<RegionKey>>() {
            Ok(pos) => {
                self.support_unit_type = Some(pos.unit.unit_type());
                self.support_from = Some(pos.region);
            }
            Err(_) => eprintln!("Invalid supported unit: {}", unit),
        }
        self
    }

    /// Specify where that unit is attempting to move
    pub fn support_move_to(&mut self, region: &str) -> &mut Self {
        let region_key = match RegionKey::from_str(region) {
            Ok(r) => r,
            Err(_) => {
                eprintln!("Invalid region: {}", region);
                return self;
            }
        };

        self.support_to = Some(region_key);
        self
    }

    /// Specify which unit is being convoyed
    pub fn convoy_unit(&mut self, unit: &str) -> &mut Self {
        match unit.parse::<UnitPosition<RegionKey>>() {
            Ok(pos) => {
                self.convoy_from = Some(pos.region);
            }
            Err(_) => eprintln!("Invalid supported unit: {}", unit),
        }
        self
    }

    /// Specify where that unit is attempting to move
    pub fn convoy_unit_to(&mut self, region: &str) -> &mut Self {
        let region_key = match RegionKey::from_str(region) {
            Ok(r) => r,
            Err(_) => {
                eprintln!("Invalid region: {}", region);
                return self;
            }
        };

        self.convoy_to = Some(region_key);
        self
    }

    // -----------------------------------------------------------------
    // BUILD FINAL ORDER
    // -----------------------------------------------------------------

    pub fn build(&mut self) -> Result<MappedMainOrder, OrderBuildError> {
        let unit_pos = self.unit_position.clone()
            .ok_or(OrderBuildError::MissingUnitPosition)?;

        // If it's a support move, build the SupportedOrder first
        if self.support_unit_type.is_some() {
            let cmd = MainCommand::Support(
                SupportedOrder::Move(
                    self.support_unit_type.take().expect("Thsi should have soemthing"),
                    self.support_from.take().expect("support_from should be set"),
                    self.support_to.take().expect("support_to should be set"),
                )
            );
            self.command = Some(cmd);
        }

        let command = self.command.clone()
            .ok_or(OrderBuildError::MissingCommand)?;

        Ok(MappedMainOrder::new_from_position(unit_pos, command))
    }
}
