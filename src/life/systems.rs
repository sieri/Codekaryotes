use crate::life::brain::Brain;
use crate::life::codekaryotes::Pos;
use crate::life::common_parts::{CodekaryoteBody, MASS_ENERGY_RATE};
use crate::life::creature::{spawn_creature, Creature};
use crate::life::creature_parts::{
    EnergyStorage, Eyes, Movement, ENERGY_MOVEMENT_RATE, ENERGY_TURNING_RATE,
};
use crate::life::genome::{CreatureGenome, Genome};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const MAX_SPEED: f32 = 100.0;
pub const MAX_ANGULAR: f32 = 3.0 * std::f32::consts::PI;
pub const ENERGY_REP_TRESH: f32 = 0.8;
pub const ENERGY_REP_COST: f32 = 0.3;

pub fn system_move_codekaryote(
    mut query: Query<(&mut ExternalForce, &mut Movement, &Transform, &Velocity)>,
) {
    for (mut force, mut movement, transform, velocity) in query.iter_mut() {
        let current_pos = transform.translation;
        movement.travelled += movement.last_pos.distance(current_pos);
        movement.last_pos = current_pos;

        let reduce_force = (1.0 - (1.0 / (MAX_SPEED - velocity.linvel.length()).exp())).max(0.0);
        let reduce_angle = (1.0 - (1.0 / (MAX_ANGULAR - velocity.angvel.abs()).exp())).max(0.0);

        let actual_forward: f32 = movement.forward
            * movement.multiplier_lin_base
            * movement.multiplier_signal
            * reduce_force;
        let actual_torque: f32 = movement.torque
            * movement.multiplier_ang_base
            * movement.multiplier_signal
            * reduce_angle;
        force.force = transform.local_x().truncate() * actual_forward;
        force.torque = actual_torque;

        movement.energy_rate =
            ENERGY_TURNING_RATE * actual_torque.abs() + actual_forward.abs() * ENERGY_MOVEMENT_RATE;
        movement.forward = 0.0;
        movement.torque = 0.0;
    }
}

pub fn system_consume_energy(
    mut query: Query<(
        &mut EnergyStorage,
        &Movement,
        &CodekaryoteBody,
        &Brain,
        &Eyes,
    )>,
) {
    for (mut energy, movement, body, brain, eyes) in query.iter_mut() {
        //consume energy
        // println!("Current energy:{}", energy.current_energy);
        energy.current_energy -= movement.energy_rate;
        // println!("energy of movement: {}", movement.energy_rate);
        energy.current_energy -= body.mass * MASS_ENERGY_RATE;
        // println!("energy of body: {}", body.mass * MASS_ENERGY_RATE);
        energy.current_energy -= brain.energy_rate;
        // println!("energy of brain: {}", brain.energy_rate);
        energy.current_energy -= eyes.energy_rate;
        // println!("energy of brain: {}", eyes.energy_rate);
        // println!(
        //     "Final Energy{}\n====================================",
        //     energy.current_energy
        // )
    }
}

pub fn system_die(mut commands: Commands, query: Query<(Entity, &EnergyStorage)>) {
    for (entity, energy) in query.iter() {
        if energy.current_energy <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn system_reproduce(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&CreatureGenome, &Transform, &mut EnergyStorage)>,
) {
    for (genome, transform, mut energy_storage) in query.iter_mut() {
        if energy_storage.get_energy_level() >= ENERGY_REP_TRESH {
            //TODO: Add randomness
            energy_storage.current_energy -= ENERGY_REP_COST * energy_storage.energy_max;
            let new_genome = genome.mutate();
            let x = transform.translation.x;
            let y = transform.translation.y;
            let baby = Creature::new(new_genome, Pos { x, y });
            spawn_creature(&mut commands, &mut meshes, &mut materials, baby);
        }
    }
}
