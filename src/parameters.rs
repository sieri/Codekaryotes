use crate::{FromWorld, World};

#[derive(Copy, Clone)]
pub struct WorldParameters {
    pub height: f32,
    pub width: f32,
    pub initial_creature: usize,
    pub initial_plant: usize,
    pub plant_per_seconds: usize,
}

impl FromWorld for WorldParameters {
    fn from_world(_world: &mut World) -> Self {
        WorldParameters {
            height: 5000.0,
            width: 5000.0,
            initial_creature: 500,
            initial_plant: 500,
            plant_per_seconds: 3,
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
}

impl FromWorld for CodekaryoteParameters {
    fn from_world(_world: &mut World) -> Self {
        CodekaryoteParameters {
            speed_factor_lowest: 100.0,
            speed_factor_highest: 200.0,
            angular_factor_lowest: 1.0,
            angular_factor_highest: 2.0,
            energy_movement_rate: 0.000005,
            energy_turning_rate: 0.0005,
            min_energy_storage_factor: 0.1,
            max_energy_storage_factor: 0.8,
        }
    }
}
