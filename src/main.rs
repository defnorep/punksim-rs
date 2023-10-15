mod global;
mod net;
mod ui;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, (ui::web_startup, net::socket_startup))
        .run();
}
