use diplomacy::{Nation, UnitPosition, geo::RegionKey, judge::MappedMainOrder, order::{ConvoyedMove, MainCommand, MoveCommand, SupportedOrder}};


#[derive(Debug)]
pub enum OrderBuildError {
    MissingUnitPosition,
    MissingCommand,
}

#[derive(Debug, Clone)]
pub struct OrderBuilder {
    nation: Nation,
    unit_position: Option<UnitPosition<'static, RegionKey>>,
    command: Option<MainCommand<RegionKey>>,
}

impl OrderBuilder {
    pub fn new(nation: &Nation) -> Self {
        Self {
            nation: nation.clone(),
            unit_position: None,
            command: None,
        }
    }

    pub fn for_unit(
        &mut self,
        unit: UnitPosition<'static, RegionKey>,
    ) -> &mut Self {
        self.unit_position = Some(unit);
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        self.unit_position = None;
        self.command = None;
        return self
    }

    pub fn for_unit_position(
        &mut self,
        pos: UnitPosition<'static, RegionKey>,
    ) -> &mut Self {
        self.unit_position = Some(pos);
        self
    }

    pub fn hold(&mut self) -> &mut Self {
        self.command = Some(MainCommand::Hold);
        self
    }

    pub fn convoy(
        &mut self,
        target: UnitPosition<'static, RegionKey>,
        to: RegionKey,
    ) -> &mut Self {
        self.command = Some(MainCommand::Convoy(
            ConvoyedMove::new(target.region, to)
        ));
        self
    }

    pub fn move_to_region(&mut self, region: RegionKey) -> &mut Self {
        self.command = Some(MainCommand::Move(MoveCommand::new(region)));
        self
    }

    pub fn support_hold(&mut self, target: UnitPosition<'static, RegionKey>) {
        self.command = Some(MainCommand::Support(
            SupportedOrder::Hold(
                target.unit.unit_type(),
                target.region,
            )
        ));
    }

    pub fn support_move(
        &mut self,
        target: UnitPosition<'static, RegionKey>,
        to: RegionKey,
    ) {
        self.command = Some(MainCommand::Support(
            SupportedOrder::Move(
                target.unit.unit_type(),
                target.region,
                to,
            )
        ));
    }

    pub fn build(self) -> Result<MappedMainOrder, OrderBuildError> {
        let unit = self
            .unit_position
            .ok_or(OrderBuildError::MissingUnitPosition)?;

        let command = self
            .command
            .ok_or(OrderBuildError::MissingCommand)?;

        Ok(MappedMainOrder::new_from_position(unit, command))
    }
}
