use super::ClockTemplate;
use crate::{Clock, SendChannel};
use askama::Template;
use bevy::prelude::*;

pub fn clock_ui(time: Res<Clock>, tx: Res<SendChannel>) {
    let html = ClockTemplate { datetime: time.0 }.render().unwrap();
    tx.0.send(html)
        .expect("Failed to send time through clock_ui channel");
}
