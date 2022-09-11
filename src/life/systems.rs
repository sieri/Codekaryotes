use crate::life::codekaryotes::Creature;
use crate::life::common_parts::{CodekaryoteBody, Parent};
use crate::life::creature_parts::{Eyes, Movement};
use crate::{Changed, EventReader, Query, Transform, Vec2, Vec4, With};
use bevy::ecs::query::QueryEntityError;
use bevy::prelude::World;
use bevy_rapier2d::na::RealField;
use bevy_rapier2d::prelude::{CollisionEvent, ExternalForce, RigidBody, Velocity};
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use std::any::{Any, TypeId};
use std::cmp::max;

pub const MAX_SPEED: f32 = 100.0;
pub const MAX_ANGULAR: f32 = 3.0 * std::f32::consts::PI;

pub fn system_move_codekaryote(
    mut query: Query<(&mut ExternalForce, &mut Movement, &Transform, &Velocity)>,
) {
    for (mut force, mut movement, transform, velocity) in query.iter_mut() {
        let current_pos = transform.translation;
        movement.travelled += movement.last_pos.distance(current_pos);
        movement.last_pos = current_pos;

        let reduce_force = (1.0 - (1.0 / (MAX_SPEED - velocity.linvel.length()).exp())).max(0.0);
        let reduce_angle = (1.0 - (1.0 / (MAX_ANGULAR - velocity.angvel.abs()).exp())).max(0.0);

        let actual_forward = movement.forward
            * movement.multiplier_lin_base
            * movement.multiplier_signal
            * reduce_force;
        let actual_torque = movement.torque
            * movement.multiplier_ang_base
            * movement.multiplier_signal
            * reduce_angle;
        force.force = transform.local_x().truncate() * actual_forward;
        force.torque = actual_torque;

        movement.forward = 0.0;
        movement.torque = 0.0;
    }
}
