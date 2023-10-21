use super::IndividualsTemplate;
use crate::{
    global::{Dimensions, Epoch, Mass},
    population::{
        hunger_system::Hunger, Alive, Attributes, CitizenBundle, CivicIdentity, Gender, Species,
    },
    time::Clock,
    SendChannel,
};
use askama::Template;
use bevy::prelude::*;

pub fn individuals_table(
    tx: Res<SendChannel>,
    clock: Res<Clock>,
    query: Query<(
        &Alive,
        &Attributes,
        &CivicIdentity,
        &Dimensions,
        &Epoch,
        &Gender,
        &Hunger,
        &Mass,
        &Species,
    )>,
) {
    let individuals: Vec<CitizenBundle> = query
        .into_iter()
        .map(
            |(alive, attr, id, dim, epoch, gender, hunger, mass, species)| CitizenBundle {
                alive: alive.clone(),
                attributes: attr.clone(),
                civic_identity: id.clone(),
                dimensions: dim.clone(),
                epoch: epoch.clone(),
                gender: gender.clone(),
                hunger: hunger.clone(),
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
