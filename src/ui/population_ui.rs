use super::IndividualsTemplate;
use crate::{
    global::{Dimensions, Mass},
    population::{CitizenBundle, CivicIdentity, Epoch, Gender, Location, Species},
    time::Clock,
    SendChannel,
};
use askama::Template;
use bevy::prelude::*;

pub fn individuals_table(
    tx: Res<SendChannel>,
    clock: Res<Clock>,
    query: Query<(
        &CivicIdentity,
        &Dimensions,
        &Epoch,
        &Gender,
        &Location,
        &Mass,
        &Species,
    )>,
) {
    let individuals: Vec<CitizenBundle> = query
        .into_iter()
        .map(
            |(civic_identity, dimensions, epoch, gender, location, mass, species)| CitizenBundle {
                civic_identity: civic_identity.clone(),
                dimensions: dimensions.clone(),
                epoch: epoch.clone(),
                gender: gender.clone(),
                location: location.clone(),
                mass: mass.clone(),
                species: species.clone(),
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
