mod graphics;
mod life;
mod parameters;
mod utils;

#[allow(dead_code, unused)]
use crate::life::{create_world, LifePlugin};
use bevy::prelude::*;
use bevy_pancam::PanCamPlugin;
use bevy_rapier2d::prelude::*;
#[allow(dead_code, unused)]
use parameters::WorldParameters;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(PanCamPlugin::default())
        .add_plugin(LifePlugin)
        //.add_plugin(RapierDebugRenderPlugin { ..default() })
        .add_system(bevy::window::close_on_esc)
        .run()
}
