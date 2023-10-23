use crate::data::Seed;
use bevy::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Resource)]
pub struct Clock(pub DateTime<Utc>);

pub fn clock_advance(delta: Res<Time>, mut time: ResMut<Clock>, seed: Res<Seed>) {
    time.0 += delta.delta().mul_f32(seed.time_multiplier);
}
