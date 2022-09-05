mod life;

use crate::life::{create_world, LifePlugin, WorldParameters};
use bevy::prelude::*;
use rapier2d::crossbeam::channel::after;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<WorldParameters>()
        .add_startup_system(create_world)
        .add_system(bevy::window::close_on_esc)
        .run()
}
