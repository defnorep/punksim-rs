use super::CitizenBundle;
use crate::data::Seed;
use bevy::prelude::{Commands, Res};
use chrono::Utc;

pub fn citizen_seeder(mut commands: Commands, seed: Res<Seed>) {
    for _ in 0..seed.population_count {
        commands.spawn(CitizenBundle::random(Utc::now(), 100.0));
    }
}
