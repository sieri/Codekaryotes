use crate::Creature;

pub type OutputCallback = fn(organism: &mut Creature, arg: f64);

pub fn get_output_callback(i: usize) -> OutputCallback {
    todo!()
}
