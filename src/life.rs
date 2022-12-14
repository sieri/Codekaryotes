use bevy::app::Plugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use creature::Creature;

use rand_distr::{Distribution, Normal};

use crate::life::brain::systems::*;
use crate::life::codekaryotes::Plant;

use crate::life::collisions::collision_event_dispatcher;
use crate::life::systems::{
    system_consume_energy, system_die, system_move_codekaryote, system_plant_spawn,
    system_reproduce,
};
use crate::parameters::{CodekaryoteParameters, WorldParameters};
use crate::{graphics, App, Commands, FromWorld, World};

//pub mod brain;
//pub mod common_parts;
//pub mod creature_parts;
pub mod genome;
//pub mod plant_parts;
mod brain;
pub mod codekaryotes;
mod collisions;
pub mod common_parts;
mod creature;
pub mod creature_parts;
mod plant;
pub mod systems;

pub struct LifePlugin;
pub struct PlantSpawnTimer(Timer);

impl Plugin for LifePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldParameters>()
            .init_resource::<CodekaryoteParameters>()
            .insert_resource(PlantSpawnTimer(Timer::from_seconds(1.0, true)))
            .add_startup_system(graphics::setup_graphics)
            .add_startup_system(create_world)
            .add_system(brain_input_system)
            .add_system(brain_push_links_system.after(brain_input_system))
            .add_system(brain_activate_system.after(brain_push_links_system))
            .add_system(brain_output_system.after(brain_activate_system))
            .add_system(system_move_codekaryote.after(brain_output_system))
            .add_system(system_consume_energy)
            .add_system(system_die)
            .add_system(system_reproduce)
            .add_system(system_plant_spawn)
            .add_system(collision_event_dispatcher);
    }
    fn name(&self) -> &str {
        "Life and evolution of Codekaryotes"
    }
}

pub fn create_world(
    mut commands: Commands,
    world_parameters: Res<WorldParameters>,
    codekaryote_parameters: Res<CodekaryoteParameters>,
    mut rapier_parameter: ResMut<RapierConfiguration>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    rapier_parameter.gravity = Vect::ZERO;

    if !world_parameters.infinite_world {
        /* Create the walls. */
        commands
            .spawn()
            .insert(Collider::cuboid(world_parameters.size, 50.0))
            .insert_bundle(TransformBundle::from(Transform::from_xyz(
                0.0,
                -world_parameters.size,
                0.0,
            )));
        commands
            .spawn()
            .insert(Collider::cuboid(world_parameters.size, 50.0))
            .insert_bundle(TransformBundle::from(Transform::from_xyz(
                0.0,
                world_parameters.size,
                0.0,
            )));
        commands
            .spawn()
            .insert(Collider::cuboid(50.0, world_parameters.size))
            .insert_bundle(TransformBundle::from(Transform::from_xyz(
                -world_parameters.size,
                0.0,
                0.0,
            )));
        commands
            .spawn()
            .insert(Collider::cuboid(50.0, world_parameters.size))
            .insert_bundle(TransformBundle::from(Transform::from_xyz(
                world_parameters.size,
                0.0,
                0.0,
            )));
    }

    let mut distribution = Normal::new(0.0, world_parameters.size / 2.0).unwrap();

    commands.insert_resource(distribution);

    /* spawn the creatures. */
    let initial_creatures = world_parameters.initial_creature;
    let initial_plants = world_parameters.initial_plant;

    for _ in 0..initial_creatures {
        let mut creature = Creature::new_rand(&mut distribution, *codekaryote_parameters);
        creature::spawn_creature(&mut commands, &mut meshes, &mut materials, creature);
    }

    for _ in 0..initial_plants {
        let mut plant = Plant::new_rand(&mut distribution, *codekaryote_parameters);
        plant::spawn_plant(&mut commands, &mut meshes, &mut materials, plant);
    }
}
