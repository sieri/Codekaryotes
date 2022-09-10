use bevy::app::Plugin;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::crossbeam::channel::after;

use crate::life::brain::systems::*;
use crate::life::codekaryotes::{Creature, Plant};
use crate::life::creature_parts::Seen;
use crate::life::systems::system_move_codekaryote;
use crate::{graphics, App, Commands, FromWorld, World};

//pub mod brain;
//pub mod common_parts;
//pub mod creature_parts;
pub mod genome;
//pub mod plant_parts;
mod brain;
pub mod codekaryotes;
pub mod common_parts;
pub mod creature_parts;
pub mod systems;

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
            .add_startup_system(create_world)
            .add_system(brain_input_system)
            .add_system(brain_push_links_system.after(brain_input_system))
            .add_system(brain_activate_system.after(brain_push_links_system))
            .add_system(brain_output_system.after(brain_activate_system))
            .add_system(system_move_codekaryote);
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

    /* Create the walls. */
    commands
        .spawn()
        .insert(Collider::cuboid(world_parameters.width, 50.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            0.0,
            -world_parameters.height,
            0.0,
        )));
    commands
        .spawn()
        .insert(Collider::cuboid(world_parameters.width, 50.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            0.0,
            world_parameters.height,
            0.0,
        )));
    commands
        .spawn()
        .insert(Collider::cuboid(50.0, world_parameters.height))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            -world_parameters.width,
            0.0,
            0.0,
        )));
    commands
        .spawn()
        .insert(Collider::cuboid(50.0, world_parameters.height))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            world_parameters.width,
            0.0,
            0.0,
        )));

    /* spawn the creatures. */
    let initial_creatures = world_parameters.initial_creature;
    let initial_plants = world_parameters.initial_plant;
    let limits = (world_parameters.width, world_parameters.height);

    for _ in 0..initial_creatures {
        let mut creature = Creature::new_rand(limits);
        let x = creature.starting_pos.x;
        let y = creature.starting_pos.y;
        let mesh_param = creature.create_mesh();
        let body_param = creature.create_body();
        let eyes_collider = creature.create_eye_sensors();

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

        let creature_entity = commands
            .spawn_bundle(creature)
            .insert(body_param.0)
            .insert(body_param.1)
            .insert(body_param.2)
            .insert(body_param.3)
            .insert(body_param.4)
            .id();

        let joint = FixedJointBuilder::new().local_anchor1(Vec2::new(0.0, 0.0));
        let eyes_entity = [commands
            .spawn()
            .insert(ImpulseJoint::new(creature_entity, joint))
            //.insert(RigidBody::Dynamic)
            .insert(eyes_collider)
            .insert(ColliderMassProperties::Mass(0.0))
            .insert(Sensor)
            .id()];

        commands
            .entity(creature_entity)
            .insert_children(0, &eyes_entity);

        //.insert(eyes_collider);
    }

    for _ in 0..initial_plants {
        let mut plant = Plant::new_rand(limits);
        let mesh_param = plant.create_mesh();
        let body_param = plant.create_body();
        plant.mesh_bundle = MaterialMesh2dBundle {
            mesh: meshes.add(mesh_param.0.into()).into(),
            material: materials.add(mesh_param.1),
            transform: Transform::from_translation(Vec3::new(
                plant.starting_pos.x,
                plant.starting_pos.y,
                0.,
            )),
            ..default()
        };

        commands
            .spawn_bundle(plant)
            .insert(body_param.0)
            .insert(body_param.1);
    }
    println!("Done!")
}
