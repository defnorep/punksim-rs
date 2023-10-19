mod data;
mod global;
mod population;
mod time;
mod transport;
mod ui;

use bevy::prelude::*;
use chrono::Utc;
use flume::{Receiver, Sender};
use population::citizen_seeder::citizen_seeder;
use time::{clock_advance, Clock};
use tokio::task;
use ui::{
    clock_ui::clock_ui, population_ui::individuals_table, sockets::socket_startup, web::web_startup,
};

#[derive(Resource)]
struct SendChannel(Sender<String>);

#[derive(Resource)]
struct ReceiveChannel(Receiver<String>);

const FIXED_TIMESTEP: f32 = 1.0;

#[tokio::main]
async fn main() {
    let (tx, rx) = flume::unbounded();

    task::spawn(socket_startup(rx));
    task::spawn(web_startup());

    let seed = data::seed();

    // set up bevy
    App::new()
        .insert_resource(SendChannel(tx))
        .insert_resource(Clock(seed.date))
        .insert_resource(seed)
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, citizen_seeder)
        .add_systems(Update, clock_advance)
        .add_systems(FixedUpdate, clock_ui)
        .add_systems(FixedUpdate, individuals_table)
        .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
        .run();
}

// just here to serve as an example on how to send messages to the socket server.
fn hello_world(tx: Res<SendChannel>) {
    tx.0.send(Utc::now().to_string()).expect("Failed to send");
}
