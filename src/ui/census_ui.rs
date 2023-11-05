use super::CensusTemplate;
use crate::{
    global::Epoch,
    population::{Census, CivicIdentity, Gender, Physiology, Vitals},
    time::Clock,
    SendChannel,
};
use askama::Template;
use bevy::prelude::*;

pub fn census_table(
    tx: Res<SendChannel>,
    clock: Res<Clock>,
    query: Query<(&CivicIdentity, &Vitals, &Gender, &Physiology, &Epoch)>,
) {
    let census = query.into_iter().fold(
        Census::empty(),
        |mut acc, (_id, alive, gender, physiology, epoch)| {
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

            let age = epoch.age(clock.0);

            match age {
                0..=2 => acc.infants += 1,
                3..=12 => acc.children += 1,
                13..=18 => acc.adolescents += 1,
                19..=64 => acc.adults += 1,
                _ => acc.elderly += 1,
            }

            acc
        },
    );

    let html = CensusTemplate {
        sets: vec![
            ("Total".into(), vec![("Total".into(), census.total)]),
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
            (
                "Age Brackets".into(),
                vec![
                    ("Infants".into(), census.infants),
                    ("Children".into(), census.children),
                    ("Adolescents".into(), census.adolescents),
                    ("Adults".into(), census.adults),
                    ("Elderly".into(), census.elderly),
                ],
            ),
        ],
    }
    .render()
    .unwrap();
    tx.0.send(html)
        .expect("Failed to send population data through population_ui channel");
}
