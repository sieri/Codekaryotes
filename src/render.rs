use bevy::ecs::system::EntityCommands;
use bevy::pbr::AlphaMode::Blend;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

pub fn spawn_graphics_sphere(
    mut command: &mut EntityCommands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    radius: f32,
) -> () {
    let internal_material = base_sphere_material(&mut materials, [0.0, 0.5, 0.0, 0.5]);
    let external_material = base_sphere_material(&mut materials, [0.0, 1.0, 0.0, 0.5]);
    // command.insert((PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::UVSphere {
    //         radius: radius / 2.0,
    //         ..default()
    //     })),
    //     material: internal_material.clone(),
    //     ..default()
    // },));
    command.insert((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius,
            ..default()
        })),
        material: external_material.clone(),
        ..default()
    },));
}

pub(crate) fn base_sphere_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    x: [f32; 4],
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: Color::from(x),
        alpha_mode: AlphaMode::Add,
        metallic: 0.1,
        reflectance: 1.0,

        ..default()
    })
}
