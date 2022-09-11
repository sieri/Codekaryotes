use crate::{Entity, World};
use bevy::ecs::component::{ComponentId, ComponentInfo};

pub fn scale_between(
    n: f32,
    smallest: f32,
    largest: f32,
    initial_smallest: Option<f32>,
    initial_biggest: Option<f32>,
) -> f32 {
    let initial_smallest = initial_smallest.unwrap_or(0.0);
    let initial_biggest = initial_biggest.unwrap_or(u32::MAX as f32);

    let factor = (initial_biggest - initial_smallest) / (largest - smallest);
    (n - initial_smallest) / factor + smallest
}
