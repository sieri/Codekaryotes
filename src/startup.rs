use crate::render::base_sphere_material;
use crate::{render, MainCamera};
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_fly_camera::FlyCamera;
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::geometry::{Collider, Restitution};
use bevy_rapier3d::math::Vect;
use bevy_rapier3d::plugin::RapierConfiguration;

pub fn setup_physics(
    mut rapier_parameter: ResMut<RapierConfiguration>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    /* Create the ground. infinite flat world */
    commands
        .spawn(Collider::halfspace(Vect::Y).unwrap())
        .insert(Restitution::coefficient(0.9));

    let radius = 0.5;
    let position_transform = Transform::from_xyz(0.0, 10.5 + radius, 0.0);
    spawn_sphere(&mut commands, meshes, materials, radius, position_transform);
}

fn spawn_sphere(
    commands: &mut Commands,
    meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    radius: f32,
    position_transform: Transform,
) {
    let mut command = commands.spawn(RigidBody::Dynamic);

    command
        .insert(Collider::ball(radius))
        .insert(Restitution::coefficient(0.9));

    render::spawn_graphics_sphere(&mut command, meshes, materials, radius);
    command.insert(TransformBundle::from(position_transform));
}

pub fn setup_graphics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add a camera so we can see the debug-render.
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        MainCamera,
        FlyCamera::default(),
    ));

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(500.0).into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });

    // let there be light!
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });
}
