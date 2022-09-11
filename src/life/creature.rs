use crate::life::codekaryotes::{Creature, Kind};
use crate::{
    default, Assets, BuildChildren, ColorMaterial, Commands, Mesh, ResMut, Transform, Vec2, Vec3,
};
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::dynamics::{FixedJointBuilder, ImpulseJoint};
use bevy_rapier2d::geometry::{ActiveEvents, ColliderMassProperties, Sensor};
use crate::life::common_parts::Parent;

pub fn spawn_creature(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    mut creature: Creature,
) {
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
        .insert(Kind::Creature)
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
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Parent{ entity: creature_entity })
        .id()];

    commands
        .entity(creature_entity)
        .insert_children(0, &eyes_entity);
}
