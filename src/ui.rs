use crate::{Clock, SendChannel};
use askama::Template;
use bevy::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Template)]
#[template(path = "partials/clock.html")]
struct ClockTemplate {
    datetime: DateTime<Utc>,
}

pub fn clock_advance(delta: Res<Time>, mut time: ResMut<Clock>) {
    time.0 += delta.delta();
}

pub fn clock_ui(time: Res<Clock>, tx: Res<SendChannel>) {
    let html = ClockTemplate { datetime: time.0 }.render().unwrap();
    tx.0.send(html)
        .expect("Failed to send time through clock_ui channel");
}
