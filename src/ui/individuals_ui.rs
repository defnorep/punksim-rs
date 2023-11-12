use super::IndividualsTemplate;
use crate::{
    global::{Dimensions, Epoch, Mass},
    population::{
        fatigue::Fatigue, hunger::Hunger, Attributes, CitizenBundle, CivicIdentity, Gender,
        Physiology, Vitals,
    },
    time::Clock,
    SendChannel,
};
use askama::Template;
use bevy::prelude::*;

// Clippy doesn't like us querying lots of components at once.
// This table won't last long anyways, it'll be too large eventually.
#[allow(clippy::type_complexity)]
pub fn individuals_table(
    tx: Res<SendChannel>,
    clock: Res<Clock>,
    query: Query<(
        &Vitals,
        &Attributes,
        &CivicIdentity,
        &Dimensions,
        &Epoch,
        &Fatigue,
        &Gender,
        &Hunger,
        &Mass,
        &Physiology,
    )>,
) {
    let individuals: Vec<CitizenBundle> = query
        .into_iter()
        .map(
            |(alive, attr, id, dim, epoch, fatigue, gender, hunger, mass, physiology)| {
                CitizenBundle {
                    vitals: alive.clone(),
                    attributes: attr.clone(),
                    civic_identity: id.clone(),
                    dimensions: dim.clone(),
                    epoch: epoch.clone(),
                    fatigue: fatigue.clone(),
                    gender: gender.clone(),
                    hunger: hunger.clone(),
                    mass: mass.clone(),
                    physiology: physiology.clone(),
                }
            },
        )
        .collect();

    let html = IndividualsTemplate {
        individuals,
        reference: clock.0,
    }
    .render()
    .unwrap();
    tx.0.send(html)
        .expect("Failed to send population data through population_ui channel");
}
