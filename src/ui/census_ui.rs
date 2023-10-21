use crate::{
    population::{Census, CivicIdentity, Gender, Species, Status},
    SendChannel,
};
use askama::Template;
use bevy::prelude::*;

use super::CensusTemplate;

pub fn census_table(tx: Res<SendChannel>, query: Query<(&CivicIdentity, &Gender, &Species)>) {
    let census = query
        .into_iter()
        .fold(Census::empty(), |mut acc, (id, gender, species)| {
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
            match id.status {
                Status::Living => acc.living += 1,
                Status::Deceased => acc.deceased += 1,
                Status::Missing => acc.missing += 1,
                Status::Unknown => acc.unknown += 1,
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
