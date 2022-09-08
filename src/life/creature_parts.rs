use crate::life::codekaryotes::Pos;
use crate::life::common_parts::ChromosomalComponent;
use crate::life::genome::{Chromosome, Mutating};
use crate::utils::scale_between;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const SPEED_FACTOR_LOWEST: f32 = 100.0;
const SPEED_FACTOR_HIGHEST: f32 = 200.0;
const ENERGY_MOVEMENT_RATE: f32 = 0.0005;

#[derive(Component, Debug, Clone)]
pub struct Movement {
    //For Module
    chromosome: Chromosome,
    //For active
    energy_rate: f32,
    //Unique
    energy_rate_base: f32,
    pub(crate) forward: f32,
    pub(crate) torque: f32,
    pub(crate) multiplier_base: f32,
    pub(crate) multiplier_signal: f32,
    pub(crate) travelled: f32,
    pub(crate) last_pos: Vec3,
}

impl ChromosomalComponent for Movement {
    fn new(c: Chromosome) -> Self {
        let multiplier_base = scale_between(
            c[0] as f32,
            SPEED_FACTOR_LOWEST,
            SPEED_FACTOR_HIGHEST,
            None,
            None,
        );
        Movement {
            chromosome: c.to_vec(),

            energy_rate: 0.0,
            energy_rate_base: ENERGY_MOVEMENT_RATE,
            forward: 0.0,
            torque: 0.0,
            multiplier_base,
            multiplier_signal: 1.0,
            travelled: 0.0,
            last_pos: Vec3::ZERO,
        }
    }

    fn get_mutated(&self) -> Chromosome {
        self.chromosome.mutate(1)
    }
}
