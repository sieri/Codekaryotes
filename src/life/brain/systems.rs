use crate::life::brain::{Activation, Brain, Inputs, Neuron, Outputs};
use crate::life::common_parts::CodekaryoteBody;
use crate::life::creature_parts::{EnergyStorage, Eyes, Movement};
use crate::parameters::CodekaryoteParameters;
use crate::Query;
use bevy::prelude::*;
use bevy_rapier2d::na::RealField;
use bevy_rapier2d::prelude::*;

pub fn brain_push_links_system(mut query: Query<&mut Brain>) {
    for mut brain in query.iter_mut() {
        let len = brain.links.len();
        for i in 0..len {
            let link = brain.links[i];
            let new_val = brain.neurons[link.input].out_val * link.weight;
            brain.neurons[link.output].write_in(new_val);
        }
    }
}

pub fn brain_activate_system(mut query: Query<&mut Brain>) {
    for mut brain in query.iter_mut() {
        brain.neurons.iter_mut().for_each(|n| -> () {
            match n.act {
                Activation::Linear => n.out_val = n.in_val,
                Activation::BinaryStep => {
                    if n.in_val > 0.0 {
                        n.out_val = 1.0;
                    } else {
                        n.out_val = 0.0;
                    }
                }
                Activation::Logistic => n.out_val = 1.0 / (1.0 + (-n.in_val).exp()),
                Activation::Tanh => n.out_val = n.in_val.tanh(),
                Activation::Gaussian => n.out_val = (-(n.in_val.powi(2))).exp(),
            };
        });
    }
}

pub fn brain_input_system(
    param: Res<CodekaryoteParameters>,
    mut query: Query<(
        &mut Brain,
        &Transform,
        &Velocity,
        &Eyes,
        &EnergyStorage,
        &CodekaryoteBody,
    )>,
) {
    for (mut brain, transform, velocity, eyes, energy, body) in query.iter_mut() {
        for i in brain.in_range() {
            let mut in_neuron: &mut Neuron = &mut brain.neurons[i];
            let in_type = in_neuron.input.unwrap();
            in_neuron.in_val = match in_type {
                Inputs::Constant => 1.0,
                Inputs::Touch => 0.0,
                Inputs::TouchForward => 0.0,
                Inputs::Angle => transform.rotation.to_axis_angle().1 / f32::pi(),
                Inputs::Speed => velocity.linvel.length() / param.max_speed,
                Inputs::RotationSpeed => velocity.angvel / f32::pi(),
                Inputs::Energy => energy.get_energy_level(),
                Inputs::NumSeen => eyes.num_seen() as f32,
                Inputs::NumSeenCreature => eyes.num_seen_creature() as f32,
                Inputs::NumSeenPlant => eyes.num_seen_plant() as f32,
                Inputs::ClosestCreatureAngle => eyes.closest_creature_dist(),
                Inputs::ClosestCreatureDist => eyes.closest_creature_angle(),
                Inputs::ClosestPlantDist => eyes.closest_plant_dist(),
                Inputs::ClosestPlantAngle => eyes.closest_plant_angle(),
                Inputs::ClosestCreatureSizeRatio => eyes.closest_creature_size() / body.size,
                Inputs::ClosestPlantSizeRatio => eyes.closest_plant_size() / body.size,
            };
        }
    }
}

pub fn brain_output_system(mut query: Query<(&Brain, &mut Movement)>) {
    for (brain, mut movement) in query.iter_mut() {
        for i in brain.out_range() {
            let out_neuron: &Neuron = &brain.neurons[i];
            let out_type = out_neuron.output.unwrap();
            let val = out_neuron.out_val;

            match out_type {
                Outputs::Multiplier => movement.multiplier_signal = val,
                Outputs::Forward => movement.forward += val,
                Outputs::Backward => movement.forward -= val,
                Outputs::TurnLeft => movement.torque -= val,
                Outputs::TurnRight => movement.torque += val,
            }
        }
    }
}
