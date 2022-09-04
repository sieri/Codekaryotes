use bevy::app::Plugin;
use crate::{App, Commands};

pub mod brain;
pub mod common_parts;
pub mod creature_parts;
pub mod genome;
pub mod plant_parts;
pub mod codekaryotes;

pub struct LifePlugin;

impl Plugin for LifePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_world);
    }

    fn name(&self) -> &str {
        "Life and evolution of Codekaryotes"
    }
}

fn create_world(mut command: Commands){
   //TODO
}