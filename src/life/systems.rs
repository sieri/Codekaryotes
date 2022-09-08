use crate::life::codekaryotes::Creature;
use crate::life::common_parts::CodekaryoteBody;
use crate::life::creature_parts::Movement;
use crate::{Changed, Query, Transform, Vec2, Vec4, With};
use bevy_rapier2d::prelude::{ExternalForce, RigidBody};

pub fn system_move_codekaryote(mut query: Query<(&mut ExternalForce, &mut Movement, &Transform)>) {
    for (mut force, mut movement, transform) in query.iter_mut() {
        let current_pos = transform.translation;
        movement.travelled += movement.last_pos.distance(current_pos);
        movement.last_pos = current_pos;
        let actual_forward =
            movement.forward * movement.multiplier_base * movement.multiplier_signal + 1.0;
        let actual_torque = movement.torque * movement.multiplier_base * movement.multiplier_signal;
        force.force = transform.local_x().truncate() * actual_forward;
        force.torque = actual_torque;
    }
}
