use bevy::app::Plugin;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::life::codekaryotes::Creature;
use crate::{graphics, App, Commands, FromWorld, World};

//pub mod brain;
//pub mod common_parts;
//pub mod creature_parts;
pub mod genome;
//pub mod plant_parts;
pub mod codekaryotes;
pub mod common_parts;

pub struct LifePlugin;

pub struct WorldParameters {
    height: f32,
    width: f32,
    pub initial_creature: usize,
    pub initial_plant: usize,
}

impl FromWorld for WorldParameters {
    fn from_world(_world: &mut World) -> Self {
        WorldParameters {
            height: 1000.0,
            width: 1000.0,
            initial_creature: 10,
            initial_plant: 10,
        }
    }
}

impl Plugin for LifePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldParameters>()
            .add_startup_system(graphics::setup_graphics)
            .add_startup_system(create_world);
    }

    fn name(&self) -> &str {
        "Life and evolution of Codekaryotes"
    }
}

pub fn create_world(
    mut commands: Commands,
    world_parameters: Res<WorldParameters>,
    mut rapier_parameter: ResMut<RapierConfiguration>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    rapier_parameter.gravity = Vect::ZERO;

    /* Create the ground. */
    commands
        .spawn()
        .insert(Collider::cuboid(world_parameters.width, 50.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            0.0,
            -world_parameters.height / 2.0,
            0.0,
        )));
    commands
        .spawn()
        .insert(Collider::cuboid(world_parameters.width, 50.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            0.0,
            world_parameters.height / 2.0,
            0.0,
        )));
    commands
        .spawn()
        .insert(Collider::cuboid(50.0, world_parameters.height))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            -world_parameters.width / 2.0,
            0.0,
            0.0,
        )));
    commands
        .spawn()
        .insert(Collider::cuboid(50.0, world_parameters.height))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            world_parameters.width / 2.0,
            0.0,
            0.0,
        )));

    let initial_creatures = world_parameters.initial_creature;
    let limits = (world_parameters.width, world_parameters.height);

    for _ in 0..initial_creatures {
        let mut creature = Creature::new_rand(limits);
        let mesh_param = creature.create_mesh();
        let body_param = creature.create_body();
        creature.mesh_bundle = MaterialMesh2dBundle {
            mesh: meshes.add(mesh_param.0.into()).into(),
            material: materials.add(mesh_param.1),
            transform: Transform::from_translation(Vec3::new(
                creature.starting_pos.x,
                creature.starting_pos.y,
                0.,
            )),
            ..default()
        };

        commands
            .spawn_bundle(creature)
            .insert(body_param.0)
            .insert(body_param.1);
    }
    println!("Done!")
}
