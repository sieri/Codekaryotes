use crate::codekaryotes::Creature;

pub type OutputCallback = fn(organism: &mut Creature, arg: f64);

fn force_multiplier_neuron(organism: &mut Creature, arg: f64) {
    organism.movement_mut().multiplier_signal += arg
}
fn move_forward_neuron(organism: &mut Creature, arg: f64) {
    println!("forward {}", arg);
    organism.movement_mut().forward += arg
}
fn move_backward_neuron(organism: &mut Creature, arg: f64) {
    println!("Backward {}", arg);
    organism.movement_mut().forward -= arg
}
fn turn_right_neuron(organism: &mut Creature, arg: f64) {
    organism.movement_mut().torque += arg
}
fn turn_left_neuron(organism: &mut Creature, arg: f64) {
    organism.movement_mut().torque -= arg
}

pub fn get_output_callback(i: usize) -> OutputCallback {
    match i {
        0 => force_multiplier_neuron,
        1 => move_forward_neuron,
        2 => move_backward_neuron,
        3 => turn_right_neuron,
        4 => turn_left_neuron,
        _ => panic!("Unkown output callback"),
    }
}
