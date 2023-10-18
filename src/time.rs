use bevy::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Resource)]
pub struct Clock(pub DateTime<Utc>);

pub fn clock_advance(delta: Res<Time>, mut time: ResMut<Clock>) {
    time.0 += delta.delta();
}
