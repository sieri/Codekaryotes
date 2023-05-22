mod render;
mod startup;
mod utils;
mod window;

use crate::window::window_panels;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_fly_camera::FlyCameraPlugin;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub(crate) struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(FlyCameraPlugin)
        .add_startup_system(startup::setup_graphics)
        .add_startup_system(startup::setup_physics)
        .add_system(print_ball_altitude)
        .add_system(window_panels.run_if(input_toggle_active(false, KeyCode::F3)))
        .run();
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}
