use diplomacy::{UnitPosition, geo::RegionKey};

use crate::{interactive::state_machine::OrderIntent, rules::order_builder::OrderBuilder};

pub trait OrderStrategy {
    fn apply(
        &self,
        unit: &UnitPosition<'static, RegionKey>,
        builder: &mut OrderBuilder,
    );
}

impl OrderStrategy for OrderIntent {
    fn apply(
        &self,
        unit: &UnitPosition<'static, RegionKey>,
        builder: &mut OrderBuilder,
    ) {
        builder.for_unit_position(unit.clone());

        match self {
            OrderIntent::Hold => {
                builder.hold();
            }

            OrderIntent::Move { to } => {
                builder.move_to_region(to.clone());
            }

            OrderIntent::SupportHold { target } => {
                builder.support_hold(target.clone());
            }

            OrderIntent::SupportMove { target, to } => {
                builder.support_move(target.clone(), to.clone());
            }

            OrderIntent::Convoy { target, to } => {
                builder.convoy(target.clone(), to.clone());
            }
        }
    }
}

