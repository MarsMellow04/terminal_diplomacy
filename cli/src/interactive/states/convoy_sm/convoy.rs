use diplomacy::judge::{Adjudicate, Context, MappedMainOrder, OrderState, ResolverState, Submission};
use diplomacy::geo::{Map, ProvinceKey, RegionKey, Terrain};
use diplomacy::judge::WillUseConvoy;
use diplomacy::order::{self, Command, MainCommand};
use diplomacy::{Order, UnitPosition, UnitType};

/// Failure cases for convoy route lookup.
pub enum ConvoyRouteError {
    /// Only armies can be convoyed.
    CanOnlyConvoyArmy,

    /// Hold, support, and convoy orders cannot be convoyed.
    CanOnlyConvoyMove,
}

/// The outcome of a convoy order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConvoyOutcome<O> {
    /// The convoy order is invalid because the convoying unit is not at sea.
    NotAtSea,
    /// The convoying unit was dislodged by another move
    Dislodged(O),
    /// The convoy was failed to resolve a paradox
    Paradox,
    /// The convoy was not disrupted. This doesn't mean the move necessarily succeeded.
    NotDisrupted,
}

impl<O> ConvoyOutcome<O> {
    /// Apply a function to any orders referenced by `self`, returning a new outcome.
    pub fn map_order<U>(self, map_fn: impl Fn(O) -> U) -> ConvoyOutcome<U> {
        use ConvoyOutcome::*;
        match self {
            NotAtSea => NotAtSea,
            Dislodged(by) => Dislodged(map_fn(by)),
            Paradox => Paradox,
            NotDisrupted => NotDisrupted,
        }
    }

    pub fn as_ref(&self) -> ConvoyOutcome<&O> {
        use ConvoyOutcome::*;
        match self {
            NotAtSea => NotAtSea,
            Dislodged(by) => Dislodged(by),
            Paradox => Paradox,
            NotDisrupted => NotDisrupted,
        }
    }
}

impl<O> From<&'_ ConvoyOutcome<O>> for OrderState {
    fn from(other: &ConvoyOutcome<O>) -> Self {
        if matches!(other, ConvoyOutcome::NotDisrupted) {
            OrderState::Succeeds
        } else {
            OrderState::Fails
        }
    }
}

impl<O> From<ConvoyOutcome<O>> for OrderState {
    fn from(other: ConvoyOutcome<O>) -> Self {
        (&other).into()
    }
}

/// Checks whether `convoy` is a valid convoy that will carry `mv_ord` from
/// its current location to the destination.
fn is_convoy_for(convoy: &MappedMainOrder, mv_ord: &MappedMainOrder) -> bool {
    match &convoy.command {
        MainCommand::Convoy(cm) => cm == mv_ord,
        _ => false,
    }
}

trait RouteStep: Eq + Clone {
    fn region(&self) -> &RegionKey;
}

impl RouteStep for &MappedMainOrder {
    fn region(&self) -> &RegionKey {
        &self.region
    }
}

impl<'a> RouteStep for UnitPosition<'a> {
    fn region(&self) -> &RegionKey {
        self.region
    }
}

/// Find all routes from `origin` to `dest` given a set of valid convoys.
fn route_steps<R: RouteStep>(
    map: &Map,
    convoys: &[R],
    origin: &ProvinceKey,
    dest: &ProvinceKey,
    working_path: Vec<R>,
) -> Vec<Vec<R>> {
    let adjacent_regions = map.find_bordering(origin);
    // Check if we have reached destination
    let destination_adjacent =
        !working_path.is_empty() &&
        adjacent_regions.iter().any(|&r| r == dest);
    // This for some reason has not worked

    if destination_adjacent {
        return vec![working_path];
    }

    let mut paths = vec![];

    for convoy in convoys {
        let already_used = working_path.contains(convoy);
        let is_adjacent = adjacent_regions.contains(&convoy.region());
        if !already_used && is_adjacent {
            let mut next_path = working_path.clone();
            next_path.push(convoy.clone());

            let mut results =
                route_steps(map, convoys, convoy.region().province(), dest, next_path);

            if !results.is_empty() {
                paths.append(&mut results);
            }
        }
    }
    paths
}

/// Checks if a convoy route may exist for an order, based on the positions
/// of fleets, the move order's source region, and the destination region.
///
/// This is used before adjudication to identify illegal orders, so it does
/// not take in a full context.
pub fn route_may_exist<'a>(
    map: &'a Map,
    unit_positions: impl IntoIterator<Item = UnitPosition<'a>>,
    mv_ord: &MappedMainOrder,
) -> bool {
    if mv_ord.unit_type == UnitType::Fleet {
        return false;
    }

    let Some(dst) = mv_ord.move_dest() else {
        return false;
    };

    let fleets = unit_positions
        .into_iter()
        .filter(|u| {
            let is_fleet = u.unit.unit_type() == UnitType::Fleet;
            let is_sea = map
                .find_region(&u.region.to_string())
                .map(|r| r.terrain() == Terrain::Sea)
                .unwrap_or(false);
            is_fleet && is_sea
        })
        .collect::<Vec<_>>();

        
    let steps = route_steps(
        map,
        &fleets,
        mv_ord.region.province(),
        dst.province(),
        vec![],
    );

    !steps.is_empty()
}

