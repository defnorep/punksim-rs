use super::CensusTemplate;
use crate::{
    population::{Census, CivicIdentity, Gender, Physiology, Vitals},
    SendChannel,
};
use askama::Template;
use bevy::prelude::*;

pub fn census_table(
    tx: Res<SendChannel>,
    query: Query<(&CivicIdentity, &Vitals, &Gender, &Physiology)>,
) {
    let census = query.into_iter().fold(
        Census::empty(),
        |mut acc, (_id, alive, gender, physiology)| {
            acc.total += 1;
            match physiology {
                Physiology::Human => acc.human += 1,
                Physiology::Android => acc.android += 1,
            }
            match gender {
                Gender::Male => acc.male += 1,
                Gender::Female => acc.female += 1,
                Gender::NonBinary => acc.non_binary += 1,
                Gender::None => acc.ungendered += 1,
            }
            match alive {
                Vitals::Alive => acc.living += 1,
                Vitals::Deceased => acc.deceased += 1,
                Vitals::Unknown => acc.unknown += 1,
            }

            acc
        },
    );

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
                "Vitals".into(),
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
