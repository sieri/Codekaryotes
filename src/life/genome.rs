pub trait Genome {}

pub type Chromonsone = Vec<i32>;

pub struct CreatureGenome {}

pub struct PlantGenome {}

impl Genome for CreatureGenome {}

impl Genome for PlantGenome {}
