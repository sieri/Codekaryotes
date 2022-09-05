mod life;
mod utils;

use crate::life::{create_world, LifePlugin, WorldParameters};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(LifePlugin)
        .add_plugin(RapierDebugRenderPlugin { ..default() })
        .add_system(bevy::window::close_on_esc)
        .run()
}
