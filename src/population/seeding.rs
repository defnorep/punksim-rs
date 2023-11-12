use super::CitizenBundle;
use crate::{data::Seed, time::Clock};
use bevy::prelude::{Commands, Res};

pub fn population_seeding_system(mut commands: Commands, clock: Res<Clock>, seed: Res<Seed>) {
    for _ in 0..seed.population_count {
        commands.spawn(CitizenBundle::random(clock.0, 100.0));
    }
}
