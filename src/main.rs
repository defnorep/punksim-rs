mod global;
mod net;

use bevy::prelude::*;
use chrono::Utc;
use flume::{Receiver, Sender};
use net::socket_startup::socket_startup;
use tokio::task;

#[derive(Resource)]
struct SendChannel(Sender<String>);

#[derive(Resource)]
struct ReceiveChannel(Receiver<String>);

const FIXED_TIMESTEP: f32 = 1.0;

#[tokio::main]
async fn main() {
    let (tx, rx) = flume::unbounded();

    task::spawn(socket_startup(rx));

    // set up bevy
    App::new()
        .insert_resource(SendChannel(tx))
        .add_plugins(MinimalPlugins)
        .add_systems(FixedUpdate, hello_world)
        .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
        .run();
}

fn hello_world(tx: Res<SendChannel>) {
    println!("{}", Utc::now().to_string());
    tx.0.send(Utc::now().to_string()).expect("Failed to send");
}
