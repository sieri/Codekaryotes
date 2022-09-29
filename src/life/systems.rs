use crate::life::brain::Brain;
use crate::life::codekaryotes::{Plant, Pos};
use crate::life::common_parts::{CodekaryoteBody, MASS_ENERGY_RATE};
use crate::life::creature::{spawn_creature, Creature};
use crate::life::creature_parts::{EnergyStorage, Eyes, Movement};
use crate::life::genome::{CreatureGenome, Genome};
use crate::life::{plant, PlantSpawnTimer};
use crate::parameters::{CodekaryoteParameters, WorldParameters};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn system_move_codekaryote(
    param: Res<CodekaryoteParameters>,
    mut query: Query<(&mut ExternalForce, &mut Movement, &Transform, &Velocity)>,
) {
    for (mut force, mut movement, transform, velocity) in query.iter_mut() {
        let current_pos = transform.translation;
        movement.travelled += movement.last_pos.distance(current_pos);
        movement.last_pos = current_pos;

        let reduce_force =
            (1.0 - (1.0 / (param.max_speed - velocity.linvel.length()).exp())).max(0.0);
        let reduce_angle =
            (1.0 - (1.0 / (param.max_angular - velocity.angvel.abs()).exp())).max(0.0);

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

        movement.energy_rate = param.energy_turning_rate * actual_torque.abs()
            + actual_forward.abs() * param.energy_movement_rate;
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
    codekaryote_param: Res<CodekaryoteParameters>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(
        &CreatureGenome,
        &Transform,
        &mut EnergyStorage,
        &CodekaryoteBody,
    )>,
) {
    for (genome, transform, mut energy_storage, body) in query.iter_mut() {
        if energy_storage.get_energy_level() >= codekaryote_param.energy_rep_tresh {
            //TODO: Add randomness
            energy_storage.current_energy -=
                codekaryote_param.energy_rep_cost * energy_storage.energy_max;
            let new_genome = genome.mutate();

            let vector_dir_offset = transform.local_x().truncate();

            let x = transform.translation.x + body.size * vector_dir_offset.x + 0.0001;
            let y = transform.translation.y + body.size * vector_dir_offset.y + 0.0001;

            let baby = Creature::new(new_genome, Pos { x, y }, *codekaryote_param);
            spawn_creature(&mut commands, &mut meshes, &mut materials, baby);
        }
    }
}

pub fn system_plant_spawn(
    mut commands: Commands,
    world_parameters: Res<WorldParameters>,
    codekaryote_parameters: Res<CodekaryoteParameters>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut timer: ResMut<PlantSpawnTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let limits = (world_parameters.width, world_parameters.height);
        for _ in 0..world_parameters.plant_per_seconds {
            let mut plant = Plant::new_rand(limits, *codekaryote_parameters);
            plant::spawn_plant(&mut commands, &mut meshes, &mut materials, plant);
        }
    }
}
