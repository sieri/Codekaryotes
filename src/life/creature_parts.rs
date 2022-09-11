use crate::life::common_parts::ChromosomalComponent;
use crate::life::genome::{Chromosome, Mutating};
use crate::utils::scale_between;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier2d::na::RealField;
use bevy_rapier2d::prelude::*;
use std::cmp::Ordering;
use std::hash::Hash;

const SPEED_FACTOR_LOWEST: f32 = 100.0;
const SPEED_FACTOR_HIGHEST: f32 = 200.0;
const ANGULAR_FACTOR_LOWEST: f32 = 1.0;
const ANGULAR_FACTOR_HIGHEST: f32 = 2.0;
const ENERGY_MOVEMENT_RATE: f32 = 0.0005;

#[derive(Component, Debug, Clone)]
pub struct Movement {
    //For Module
    chromosome: Chromosome,
    //Unique
    energy_rate_base: f32,
    pub(crate) forward: f32,
    pub(crate) torque: f32,
    pub(crate) multiplier_lin_base: f32,
    pub(crate) multiplier_ang_base: f32,
    pub(crate) multiplier_signal: f32,
    pub(crate) travelled: f32,
    pub(crate) last_pos: Vec3,
}

impl ChromosomalComponent for Movement {
    fn new(c: Chromosome) -> Self {
        let multiplier_lin_base = scale_between(
            c[0] as f32,
            SPEED_FACTOR_LOWEST,
            SPEED_FACTOR_HIGHEST,
            None,
            None,
        );
        let multiplier_ang_base = scale_between(
            c[1] as f32,
            ANGULAR_FACTOR_LOWEST,
            ANGULAR_FACTOR_HIGHEST,
            None,
            None,
        );
        Movement {
            chromosome: c.to_vec(),
            energy_rate_base: ENERGY_MOVEMENT_RATE,
            forward: 0.0,
            torque: 0.0,
            multiplier_lin_base,
            multiplier_ang_base,
            multiplier_signal: 1.0,
            travelled: 0.0,
            last_pos: Vec3::ZERO,
        }
    }

    fn get_mutated(&self) -> Chromosome {
        self.chromosome.mutate(1)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Seen {
    pub(crate) position: Vec3,
    pub(crate) dist: f32,
    pub(crate) angle: f32,
    pub(crate) size: f32,
}

impl Seen {
    pub fn new(this_position: Vec3, other_position: Vec3, other_size: f32) -> Self {
        Seen {
            position: other_position,
            dist: this_position.distance(other_position),
            angle: this_position.angle_between(other_position),
            size: other_size,
        }
    }
}

impl Eq for Seen {}

impl PartialEq<Self> for Seen {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialOrd<Self> for Seen {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl Ord for Seen {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.dist < other.dist {
            Ordering::Less
        } else if self.dist > other.dist {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Eyes {
    genome: Chromosome,
    energy_rate: f32,
    pub(crate) seen_creature: HashMap<u32, Seen>,
    pub(crate) seen_plants: HashMap<u32, Seen>,
    pub(crate) fov: f32,
    pub(crate) range: f32,
}

impl ChromosomalComponent for Eyes {
    fn new(c: Chromosome) -> Self {
        //TODO: Set parameters
        const EYE_RANGE_LIMIT: f32 = 300.0;
        const ENERGY_EYES_RATE: f32 = 0.005;

        let fov = scale_between(c[0] as f32, 0.002, f32::two_pi(), None, None);
        let range = scale_between(c[1] as f32, 40.0, EYE_RANGE_LIMIT, None, None);
        let energy_rate: f32 = (fov / 180.0 * range) * ENERGY_EYES_RATE;

        Eyes {
            genome: c.to_vec(),
            energy_rate,
            seen_creature: HashMap::new(),
            seen_plants: HashMap::new(),
            fov,
            range,
        }
    }

    fn get_mutated(&self) -> Chromosome {
        self.genome.mutate(1)
    }
}

impl Eyes {
    pub fn num_seen(&self) -> usize {
        self.seen_creature.len() + self.seen_plants.len()
    }

    pub fn num_seen_creature(&self) -> usize {
        self.seen_plants.len()
    }

    pub fn num_seen_plant(&self) -> usize {
        self.seen_plants.len()
    }

    pub fn closest_creature_dist(&self) -> f32 {
        match self.seen_creature.values().min() {
            Some(s) => s.dist,
            None => 0.0,
        }
    }

    pub fn closest_creature_angle(&self) -> f32 {
        match self.seen_creature.values().min() {
            Some(s) => s.angle,
            None => 0.0,
        }
    }

    pub fn closest_plant_dist(&self) -> f32 {
        match self.seen_creature.values().min() {
            Some(s) => s.dist,
            None => 0.0,
        }
    }

    pub fn closest_plant_angle(&self) -> f32 {
        match self.seen_plants.values().min() {
            Some(s) => s.angle,
            None => 0.0,
        }
    }
}
