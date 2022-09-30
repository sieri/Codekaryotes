use crate::life::common_parts;
use crate::life::common_parts::{ChromosomalComponent, CodekaryoteBody};
use crate::life::genome::{Chromosome, Mutating};
use crate::parameters::CodekaryoteParameters;
use crate::utils::scale_between;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier2d::na::RealField;
use bevy_rapier2d::prelude::*;
use std::cmp::Ordering;

#[derive(Component, Debug, Clone)]
pub struct Movement {
    //For Module
    chromosome: Chromosome,
    pub(crate) energy_rate: f32,
    pub(crate) forward: f32,
    pub(crate) torque: f32,
    pub(crate) multiplier_lin_base: f32,
    pub(crate) multiplier_ang_base: f32,
    pub(crate) multiplier_signal: f32,
    pub(crate) travelled: f32,
    pub(crate) last_pos: Vec3,
}

impl ChromosomalComponent for Movement {
    fn new(c: Chromosome, param: CodekaryoteParameters) -> Self {
        let multiplier_lin_base = scale_between(
            c[0] as f32,
            param.speed_factor_lowest,
            param.speed_factor_highest,
            None,
            None,
        );
        let multiplier_ang_base = scale_between(
            c[1] as f32,
            param.angular_factor_lowest,
            param.angular_factor_highest,
            None,
            None,
        );
        Movement {
            chromosome: c.to_vec(),
            energy_rate: 0.0,
            forward: 0.0,
            torque: 0.0,
            multiplier_lin_base,
            multiplier_ang_base,
            multiplier_signal: 1.0,
            travelled: 0.0,
            last_pos: Vec3::ZERO,
        }
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
    pub(crate) energy_rate: f32,
    pub(crate) seen_creature: HashMap<u32, Seen>,
    pub(crate) seen_plants: HashMap<u32, Seen>,
    pub(crate) fov: f32,
    pub(crate) range: f32,
}

impl ChromosomalComponent for Eyes {
    fn new(c: Chromosome, param: CodekaryoteParameters) -> Self {
        let fov = scale_between(c[0] as f32, 0.002, f32::two_pi(), None, None);
        let range = scale_between(c[1] as f32, 40.0, param.eye_range_limit, None, None);
        let energy_rate: f32 = (fov / 180.0 * range) * param.energy_eyes_rate;

        Eyes {
            genome: c.to_vec(),
            energy_rate,
            seen_creature: HashMap::new(),
            seen_plants: HashMap::new(),
            fov,
            range,
        }
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

    pub fn closest_creature_size(&self) -> f32 {
        match self.seen_creature.values().min() {
            Some(s) => s.size,
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

    pub fn closest_plant_size(&self) -> f32 {
        match self.seen_plants.values().min() {
            Some(s) => s.size,
            None => 0.0,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct EnergyStorage {
    chromosome: Chromosome,
    pub current_energy: f32,
    pub energy_max: f32,
    size_factor: f32,
}

impl ChromosomalComponent for EnergyStorage {
    fn new(c: Chromosome, param: CodekaryoteParameters) -> Self {
        EnergyStorage {
            chromosome: c.to_vec(),
            current_energy: 0.0,
            energy_max: 0.0,
            size_factor: scale_between(
                c[0] as f32,
                param.min_energy_storage_factor,
                param.max_energy_storage_factor,
                None,
                None,
            ),
        }
    }
}

impl EnergyStorage {
    pub fn init(&mut self, body: CodekaryoteBody) {
        let max_storage = body.mass * common_parts::MASS_ENERGY * self.size_factor;
        self.energy_max = max_storage;
        self.current_energy = max_storage / 2.0;
    }

    pub fn add_energy(&mut self, en: f32) {
        self.current_energy += en;

        if self.current_energy > self.energy_max {
            self.current_energy = self.energy_max;
        }
    }

    pub fn get_energy_level(&self) -> f32 {
        self.current_energy / self.energy_max
    }
}
