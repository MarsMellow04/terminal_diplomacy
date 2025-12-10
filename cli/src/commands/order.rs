use crate::Command;
use crate::interactive::states::show_units::{self, ShowUnitState};
use diplomacy::{ShortName, Unit, UnitPosition, UnitType};
use diplomacy::{geo::RegionKey};
use diplomacy::judge::{Rulebook, Submission, MappedMainOrder};
use diplomacy::geo::{Map, Terrain, standard_map};
use serde::Deserialize;
use serde_json::{json};
use std::collections::BTreeMap; // <-- correct map type for string->string JSON

use crate::interactive::state_machine::StateMachine;

pub enum GamePhase {
    SpringMovement,
    SpringRetreat,
    FallMovement,
    FallRetreat,
    WinterBuild,
}

#[derive(Deserialize)]
struct MovesJson {
    edition: Option<String>,
    orders: Vec<String>,
}

#[derive(Default)]
pub struct OrderCommand {
    name: Option<String>,
}

impl OrderCommand {
    pub fn new(name: Option<String>) -> Self {
        Self { name }
    }

    fn parse_flags(&self) -> Option<String> {
        println!("Checking flags... ");
        if self.name.is_none() {
            println!("Meowza the flags are empty");
            return None
        }
        None
    }

    fn get_the_units(&self) {
        // demo position
        // let pos: UnitPosition<'_, RegionKey> = "FRA: F bre".parse().unwrap();
        // println!("{pos}");

        // Dummy Positions
        let positions: Vec<UnitPosition<'_, RegionKey>> = vec![
            "FRA: F bre",
            "FRA: A par",
            "FRA: A mar",
        ].into_iter()
        .map(|ord| ord.parse().unwrap()).collect();

        let first_pos = positions[0].clone();
        let second_pos: UnitPosition<'_, RegionKey> = "FRA: A bre".parse().unwrap();

        println!("\n\nThese are your moves: \n\n");
        for position in positions {
            println!("{position}");
        }
        let map:&Map = standard_map();

        let possible_terrains =  match first_pos.unit.unit_type() {
            UnitType::Army => { [Terrain::Land, Terrain::Coast]}
            UnitType::Fleet => {[Terrain::Sea,Terrain::Coast, ]}
        };

        let more_possible =  match second_pos.unit.unit_type() {
            UnitType::Army => { [Terrain::Land, Terrain::Coast]}
            UnitType::Fleet => {[Terrain::Sea,Terrain::Coast, ]}
        };


        let correct_bordering_nations: Vec<&RegionKey> = map.find_bordering(&first_pos.region)
            .into_iter()
            .filter(|region_key| {
                // Filter so it is only possible terrains
                let region = map.find_region(&region_key.short_name()).unwrap();
                possible_terrains.contains(&region.terrain())
            }).collect();

        for bordering_nation in correct_bordering_nations {
            println!("{bordering_nation}");
        }

        println!("\n\n Now for an army: \n\n");

        let correct_bordering_nations: Vec<&RegionKey> = map.find_bordering(&second_pos.region)
            .into_iter()
            .filter(|region_key| {
                // Filter so it is only possible terrains
                let region = map.find_region(&region_key.short_name()).unwrap();
                more_possible.contains(&region.terrain())
            }).collect();

        for bordering_nation in correct_bordering_nations {
            println!("{bordering_nation}");
        }
        


        


        let orders: Vec<MappedMainOrder> = vec![
            "TUR: F ank -> bla",
            "TUR: A bul -> rum",
            "TUR: A smy -> ank",
            "TUR: A con -> bul",
        ]
        .into_iter()
        .map(|ord| ord.parse().unwrap())
        .collect();

        let submission = Submission::with_inferred_state(standard_map(), orders);
        let outcome = submission.adjudicate(Rulebook::default());

        let results: BTreeMap<String, String> = outcome
            .all_orders_with_outcomes()
            .map(|(order, result)| (order.to_string(), format!("{result:?}")))
            .collect();

        println!("{}", serde_json::to_string_pretty(&results).unwrap());
    }

    fn launch_move_interactive(&self) {
        self.get_the_units()
    }
}

impl Command for OrderCommand {
    fn execute(&self) -> bool {
        if self.parse_flags().is_some() {
            println!("Flags have been added! will skip interactive method")
        }
        use std::io::{self, Write};
        let mut machine = StateMachine::new(
            Box::new(ShowUnitState::new()),
            vec!["FRA: F bre", "FRA: A par","FRA: A mos", "FRA: A bur"]
                .into_iter()
                .map(|str | str.to_string())
                .collect()
        );
        while !machine.is_finished() {
            machine.state.render(&machine);

            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            machine.update(input.trim());
        }

        println!("Final orders = {:?}", machine.data.orders);
        true
    }
}
