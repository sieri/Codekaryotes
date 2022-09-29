use crate::{Camera2dBundle, Commands};
use bevy_pancam::PanCam;

pub fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(PanCam::default());
}
