use crate::life::codekaryotes::{Kind, Plant};
use crate::{default, Assets, ColorMaterial, Commands, Mesh, ResMut, Transform, Vec3};
use bevy::sprite::MaterialMesh2dBundle;

pub fn spawn_plant(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    mut plant: Plant,
) {
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
        .insert(Kind::Plant)
        .insert(body_param.0)
        .insert(body_param.1);
}
