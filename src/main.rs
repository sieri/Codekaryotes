mod life;

use crate::life::{create_world, LifePlugin, WorldParameters};
use bevy::prelude::*;
use rapier2d::crossbeam::channel::after;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LifePlugin)
        .add_system(bevy::window::close_on_esc)
        .run()
}
