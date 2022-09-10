use crate::life::brain::Brain;
use crate::life::common_parts::{ChromosomalComponent, CodekaryoteBody, CodekaryoteColor};
use crate::life::creature_parts::{Eyes, Movement};
use crate::life::genome::{CreatureGenome, PlantGenome};
use crate::shape::Circle;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
};
use bevy_rapier2d::prelude::*;
use rand::Rng;

#[derive(Component, Debug, Copy, Clone)]
pub struct Pos {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Bundle, Clone)]
pub struct Creature {
    pub(crate) starting_pos: Pos,
    pub(crate) color: CodekaryoteColor,
    pub(crate) body: CodekaryoteBody,
    pub(crate) movement: Movement,
    pub(crate) eyes: Eyes,
    pub(crate) brain: Brain,
    #[bundle]
    pub mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
}

#[derive(Bundle)]
pub struct EyeBundle {
    coll: Collider,
    sens: Sensor,
}

impl Creature {
    pub fn new(genome: CreatureGenome, pos: Pos) -> Self {
        Creature {
            starting_pos: pos,
            color: CodekaryoteColor::new(genome.color),
            body: CodekaryoteBody::new(genome.body),
            movement: Movement::new(genome.movement),
            eyes: Eyes::new(genome.eyes),
            brain: Brain::new(genome.brain),
            mesh_bundle: default(),
        }
    }

    pub fn new_rand(limits: (f32, f32)) -> Self {
        Self::new(CreatureGenome::new(), Pos::rand(limits))
    }

    pub fn create_mesh(&self) -> (Circle, ColorMaterial) {
        let color = Color::rgb(self.color.r, self.color.g, self.color.b);
        let circle = Circle::new(self.body.size);
        let material = ColorMaterial::from(color);
        (circle, material)
    }

    pub fn create_body(&self) -> (RigidBody, Collider, ExternalForce, Velocity, Damping) {
        (
            RigidBody::Dynamic,
            Collider::ball(self.body.size),
            ExternalForce {
                force: Vec2::ZERO,
                torque: 0.0,
            },
            Velocity::zero(),
            Damping {
                linear_damping: 0.5,
                angular_damping: 0.9,
            },
        )
    }

    pub fn create_eye_sensors(&self) -> Collider {
        let range = self.eyes.range;
        let mut vertex = vec![Vec2::ZERO];
        let fov = self.eyes.fov;
        let half_fov = fov / 2.0;
        let num_seg = 20;
        for i in 0..=num_seg {
            let angle = ((i as f32) * fov / num_seg as f32) - half_fov;
            let vec = range * Vec2::new(angle.cos(), angle.sin());

            vertex.push(vec)
        }
        let mut coll = Collider::convex_polyline(vertex).unwrap();
        coll
    }
}

#[derive(Bundle, Clone)]
pub struct Plant {
    pub(crate) starting_pos: Pos,
    pub(crate) color: CodekaryoteColor,
    pub(crate) body: CodekaryoteBody,
    #[bundle]
    pub mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
}

impl Plant {
    pub fn new(genome: PlantGenome, pos: Pos) -> Self {
        Plant {
            starting_pos: pos,
            color: CodekaryoteColor::new(genome.color),
            body: CodekaryoteBody::new(genome.body),
            mesh_bundle: default(),
        }
    }

    pub fn new_rand(limits: (f32, f32)) -> Self {
        Self::new(PlantGenome::new(), Pos::rand(limits))
    }

    pub fn create_mesh(&self) -> (Circle, ColorMaterial) {
        let color = Color::rgb(self.color.r, self.color.g, self.color.b);
        let circle = Circle::new(self.body.size);
        let material = ColorMaterial::from(color);
        (circle, material)
    }

    pub fn create_body(&self) -> (RigidBody, Collider) {
        (RigidBody::Dynamic, Collider::ball(self.body.size))
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
