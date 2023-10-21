mod data;
mod global;
mod population;
mod time;
mod ui;

use bevy::prelude::*;
use flume::{Receiver, Sender};
use population::{
    hunger_system::{hunger_advance, hunger_setup},
    population_seeding_system::population_seeding_system,
};
use time::{clock_advance, Clock};
use tokio::task;
use ui::{
    census_ui::census_table, clock_ui::clock_ui, individuals_ui::individuals_table,
    sockets::socket_startup, web::web_startup,
};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[derive(Resource)]
struct SendChannel(Sender<String>);

#[derive(Resource)]
struct ReceiveChannel(Receiver<String>);

const FIXED_TIMESTEP: f32 = 1.0;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

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
        .add_systems(Startup, (population_seeding_system, hunger_setup))
        .add_systems(Update, (clock_advance, hunger_advance))
        .add_systems(FixedUpdate, (clock_ui, individuals_table, census_table))
        .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
        .run();
}
