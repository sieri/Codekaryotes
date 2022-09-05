use bevy::ecs::schedule::ShouldRun::No;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
};

use crate::life::common_parts::ChromosomalComponent;
use crate::life::genome::{CreatureGenome, Genome};
use crate::life::{common_parts, WorldParameters};
use crate::shape::Circle;
use crate::Res;
use rand::Rng;

#[derive(Component, Debug, Copy, Clone)]
pub struct Pos {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Bundle, Clone)]
pub struct Creature {
    pub(crate) pos: Pos,
    pub(crate) color: common_parts::Color,
    #[bundle]
    pub mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
}

#[derive(Bundle, Clone)]
pub struct Plant {
    pos: Pos,
}

impl Creature {
    pub fn new(genome: CreatureGenome, pos: Pos) -> Self {
        Creature {
            pos,
            color: common_parts::Color::new(genome.color),
            mesh_bundle: default(),
        }
    }

    pub fn new_rand(limits: (f32, f32)) -> Self {
        Self::new(CreatureGenome::new(), Pos::rand(limits))
    }

    pub fn create_mesh(&self) -> (Circle, ColorMaterial) {
        let color = Color::rgb(self.color.r, self.color.g, self.color.b);
        let circle = shape::Circle::new(50.);
        let material = ColorMaterial::from(color);
        (circle, material)
    }
}

impl Pos {
    pub fn rand(limits: (f32, f32)) -> Pos {
        let mut r = rand::thread_rng();
        Pos {
            x: r.gen_range(-(limits.0 / 2.0)..(limits.0 / 2.0)),
            y: r.gen_range(-(limits.1 / 2.0)..(limits.1 / 2.0)),
        }
    }

    pub(crate) fn dist(&self, other: Self) -> f32 {
        ((self.x - other.x).powi(2) + (self.x - other.x).powi(2)).sqrt()
    }

    pub(crate) fn angle(&self, other: Self) -> f32 {
        (other.y - self.y).atan2(other.x - self.x)
    }
}
