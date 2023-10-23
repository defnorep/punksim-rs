use super::CensusTemplate;
use crate::{
    population::{Census, CivicIdentity, Gender, LivingStatus, Species},
    SendChannel,
};
use askama::Template;
use bevy::prelude::*;

pub fn census_table(
    tx: Res<SendChannel>,
    query: Query<(&CivicIdentity, &LivingStatus, &Gender, &Species)>,
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
                    LivingStatus::Alive => acc.living += 1,
                    LivingStatus::Deceased => acc.deceased += 1,
                    LivingStatus::Unknown => acc.unknown += 1,
                }

                return acc;
            });

    let html = CensusTemplate {
        sets: vec![
            (
                "Physiology".into(),
                vec![
                    ("Human".into(), census.human),
                    ("Android".into(), census.android),
                ],
            ),
            (
                "Sexual Identity".into(),
                vec![
                    ("Male".into(), census.male),
                    ("Female".into(), census.female),
                    ("Non-Binary".into(), census.non_binary),
                    ("Ungendered".into(), census.ungendered),
                ],
            ),
            (
                "Life Signs".into(),
                vec![
                    ("Living".into(), census.living),
                    ("Deceased".into(), census.deceased),
                    ("Unknown".into(), census.unknown),
                ],
            ),
        ],
    }
    .render()
    .unwrap();
    tx.0.send(html)
        .expect("Failed to send population data through population_ui channel");
}
