mod data;
mod global;
mod population;
mod time;
mod ui;

use bevy::prelude::*;
use flume::{Receiver, Sender};
use population::{hunger::hunger_advance, seeding::population_seeding_system};
use time::{clock_advance, Clock};
use tokio::task;
use ui::{
    census_ui::census_table, clock_ui::clock_ui, individuals_ui::individuals_table,
    network::network_startup,
};

#[derive(Resource)]
struct SendChannel(Sender<String>);

#[derive(Resource)]
struct ReceiveChannel(Receiver<String>);

const FIXED_TIMESTEP: f32 = 1.0;

#[tokio::main]
async fn main() {
    let (tx, rx) = flume::unbounded();

    task::spawn(network_startup(rx));

    let seed = data::seed();

    // set up bevy
    App::new()
        .insert_resource(SendChannel(tx))
        .insert_resource(Clock(seed.date))
        .insert_resource(seed)
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, population_seeding_system)
        .add_systems(Update, (clock_advance, hunger_advance))
        .add_systems(FixedUpdate, (clock_ui, individuals_table, census_table))
        .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
        .run();
}
