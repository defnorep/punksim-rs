use super::IndividualsTemplate;
use crate::{
    global::{Dimensions, Mass},
    population::{CitizenBundle, CivicIdentity, Epoch, Gender, Location, Species},
    SendChannel,
};
use askama::Template;
use bevy::prelude::*;

pub fn individuals_table(
    tx: Res<SendChannel>,
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

    let html = IndividualsTemplate { individuals }.render().unwrap();
    tx.0.send(html)
        .expect("Failed to send time through clock_ui channel");
}
