use crate::life::codekaryotes::Creature;
use crate::life::creature_parts::{ActiveModule, Eyes};

pub type InputCallback = fn(organism: &mut Creature) -> f64;

fn constant(organism: &mut Creature) -> f64 {
    1.0
}

fn touch(organism: &mut Creature) -> f64 {
    organism.touch_mut().touch as f64
}

fn touch_forward(organism: &mut Creature) -> f64 {
    organism.touch_mut().touch_forward as f64
}

fn angle(organism: &mut Creature) -> f64 {
    organism.body_mut().get_angle()
}

fn speed(organism: &mut Creature) -> f64 {
    organism.body_mut().get_speed()
}

fn rotation_speed(organism: &mut Creature) -> f64 {
    organism.body_mut().get_speed_rotation()
}

fn energy(organism: &mut Creature) -> f64 {
    organism.energy_storage_mut().energy
}

fn num_seen(organism: &mut Creature) -> f64 {
    (organism.eyes_mut().seen_plants.len() + organism.eyes_mut().seen_creatures.len()) as f64
}

fn num_seen_creature(organism: &mut Creature) -> f64 {
    organism.eyes_mut().seen_creatures.len() as f64
}

fn num_seen_plant(organism: &mut Creature) -> f64 {
    organism.eyes_mut().seen_plants.len() as f64
}

fn closest_creature_angle(organism: &mut Creature) -> f64 {
    let eyes = Creature::eyes(organism);
    Eyes::closest_creature_angle(eyes, organism)
}

fn closest_plant_angle(organism: &mut Creature) -> f64 {
    let eyes = Creature::eyes(organism);
    Eyes::closest_plant_angle(eyes, organism)
}

pub(crate) fn get_input_callback(i: usize) -> InputCallback {
    match i {
        00 => constant,
        01 => touch,
        02 => touch_forward,
        03 => angle,
        04 => speed,
        05 => rotation_speed,
        06 => energy,
        07 => num_seen,
        08 => num_seen_creature,
        09 => num_seen_plant,
        10 => closest_creature_angle,
        11 => closest_plant_angle,
        _ => panic!("Trying to get an input type that doesn't exist"),
    }
}
