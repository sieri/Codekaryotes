use crate::{FromWorld, World};

#[derive(Copy, Clone)]
pub struct WorldParameters {
    pub size: f32,
    pub initial_creature: usize,
    pub initial_plant: usize,
    pub plant_per_seconds: usize,
    pub infinite_world: bool,
}

impl FromWorld for WorldParameters {
    fn from_world(_world: &mut World) -> Self {
        WorldParameters {
            size: 5000.0,
            initial_creature: 500,
            initial_plant: 500,
            plant_per_seconds: 3,
            infinite_world: true,
        }
    }
}

#[derive(Copy, Clone)]
pub struct CodekaryoteParameters {
    pub speed_factor_lowest: f32,
    pub speed_factor_highest: f32,
    pub angular_factor_lowest: f32,
    pub angular_factor_highest: f32,
    pub energy_movement_rate: f32,
    pub energy_turning_rate: f32,
    pub min_energy_storage_factor: f32,
    pub max_energy_storage_factor: f32,
    pub eye_range_limit: f32,
    pub energy_eyes_rate: f32,
    pub energy_rep_tresh: f32,
    pub energy_rep_cost: f32,
    pub max_angular: f32,
    pub max_speed: f32,
}

impl FromWorld for CodekaryoteParameters {
    fn from_world(_world: &mut World) -> Self {
        //create default values
        CodekaryoteParameters {
            //Physics parameters
            speed_factor_lowest: 100.0,
            speed_factor_highest: 200.0,
            angular_factor_lowest: 1.0,
            angular_factor_highest: 2.0,
            max_speed: 100.0,
            max_angular: 3.0 * std::f32::consts::PI,

            //Eyes
            eye_range_limit: 300.0,
            energy_eyes_rate: 0.005,

            //energy tuning parameters
            energy_movement_rate: 0.000005,
            energy_turning_rate: 0.0005,
            min_energy_storage_factor: 0.1,
            max_energy_storage_factor: 0.8,
            energy_rep_tresh: 0.8,
            energy_rep_cost: 0.3,
        }
    }
}
