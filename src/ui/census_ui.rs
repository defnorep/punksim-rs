use super::CensusTemplate;
use crate::{
    population::{Alive, Census, CivicIdentity, Gender, Species},
    SendChannel,
};
use askama::Template;
use bevy::prelude::*;

pub fn census_table(
    tx: Res<SendChannel>,
    query: Query<(&CivicIdentity, &Alive, &Gender, &Species)>,
) {
    let census =
        query
            .into_iter()
            .fold(Census::empty(), |mut acc, (_id, alive, gender, species)| {
                acc.total += 1;
                match species {
                    Species::Human => acc.human += 1,
                    Species::Android => acc.android += 1,
                }
                match gender {
                    Gender::Male => acc.male += 1,
                    Gender::Female => acc.female += 1,
                    Gender::NonBinary => acc.non_binary += 1,
                    Gender::None => acc.ungendered += 1,
                }
                match alive {
                    Alive::Alive => acc.living += 1,
                    Alive::Deceased => acc.deceased += 1,
                    Alive::Unknown => acc.unknown += 1,
                }

                return acc;
            });

    let html = CensusTemplate {
        total: census.total,
        human: census.human,
        android: census.android,
        male: census.male,
        female: census.female,
        non_binary: census.non_binary,
        ungendered: census.ungendered,
        living: census.living,
        deceased: census.deceased,
        missing: census.missing,
    }
    .render()
    .unwrap();
    tx.0.send(html)
        .expect("Failed to send population data through population_ui channel");
}
