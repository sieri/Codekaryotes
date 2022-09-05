use bevy::app::Plugin;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::life::codekaryotes::Creature;
use crate::{App, Commands, FromWorld, World};

//pub mod brain;
//pub mod common_parts;
//pub mod creature_parts;
pub mod genome;
//pub mod plant_parts;
pub mod body;
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
    fn from_world(world: &mut World) -> Self {
        WorldParameters {
            height: 500.0,
            width: 500.0,
            initial_creature: 10,
            initial_plant: 10,
        }
    }
}

impl Plugin for LifePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldParameters>()
            .add_startup_system(create_world);
    }

    fn name(&self) -> &str {
        "Life and evolution of Codekaryotes"
    }
}

pub fn create_world(
    mut commands: Commands,
    world_parameters: Res<WorldParameters>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    let initial_creatures = world_parameters.initial_creature;
    let limits = (world_parameters.width, world_parameters.height);

    for _ in 0..initial_creatures {
        let mut creature = Creature::new_rand(limits);
        let mesh_param = creature.create_mesh();
        creature.mesh_bundle = MaterialMesh2dBundle {
            mesh: meshes.add(mesh_param.0.into()).into(),
            material: materials.add(mesh_param.1),
            transform: Transform::from_translation(Vec3::new(creature.pos.x, creature.pos.y, 0.)),
            ..default()
        };
        commands.spawn_bundle(creature);
    }
    println!("Done!")
}
