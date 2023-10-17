mod global;
mod net;
mod ui;

use bevy::prelude::*;
use chrono::{DateTime, Utc};
use flume::{Receiver, Sender};
use net::{socket_startup::socket_startup, web_startup};
use tokio::task;
use ui::{clock_advance, clock_ui};

#[derive(Resource)]
struct SendChannel(Sender<String>);

#[derive(Resource)]
struct ReceiveChannel(Receiver<String>);

#[derive(Resource)]
struct Clock(DateTime<Utc>);

const FIXED_TIMESTEP: f32 = 1.0;

#[tokio::main]
async fn main() {
    let (tx, rx) = flume::unbounded();

    task::spawn(socket_startup(rx));
    task::spawn(web_startup());

    // set up bevy
    App::new()
        .insert_resource(SendChannel(tx))
        .insert_resource(Clock(Utc::now()))
        .add_plugins(MinimalPlugins)
        .add_systems(Update, clock_advance)
        .add_systems(FixedUpdate, clock_ui)
        .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
        .run();
}

fn hello_world(tx: Res<SendChannel>) {
    println!("{}", Utc::now().to_string());
    tx.0.send(Utc::now().to_string()).expect("Failed to send");
}
