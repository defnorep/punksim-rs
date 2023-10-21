use super::IndividualsTemplate;
use crate::{
    global::{Dimensions, Mass},
    population::{Attributes, CitizenBundle, CivicIdentity, Epoch, Gender, Species},
    time::Clock,
    SendChannel,
};
use askama::Template;
use bevy::prelude::*;

pub fn individuals_table(
    tx: Res<SendChannel>,
    clock: Res<Clock>,
    query: Query<(
        &Attributes,
        &CivicIdentity,
        &Dimensions,
        &Epoch,
        &Gender,
        &Mass,
        &Species,
    )>,
) {
    let individuals: Vec<CitizenBundle> = query
        .into_iter()
        .map(
            |(attributes, civic_identity, dimensions, epoch, gender, mass, species)| {
                CitizenBundle {
                    attributes: attributes.clone(),
                    civic_identity: civic_identity.clone(),
                    dimensions: dimensions.clone(),
                    epoch: epoch.clone(),
                    gender: gender.clone(),
                    mass: mass.clone(),
                    species: species.clone(),
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
