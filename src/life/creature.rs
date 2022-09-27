use crate::life::brain::Brain;
use crate::life::codekaryotes::{Kind, Pos};
use crate::life::common_parts::{ChromosomalComponent, CodekaryoteBody, CodekaryoteColor, Parent};
use crate::life::creature_parts::{EnergyStorage, Eyes, Movement};
use crate::life::genome::CreatureGenome;
use crate::parameters::CodekaryoteParameters;
use crate::shape::Circle;
use crate::{
    default, Assets, BuildChildren, Color, ColorMaterial, Commands, Mesh, ResMut, Transform, Vec2,
    Vec3,
};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::dynamics::{
    Damping, ExternalForce, FixedJointBuilder, ImpulseJoint, RigidBody, Velocity,
};
use bevy_rapier2d::geometry::{ActiveEvents, Collider, ColliderMassProperties, Sensor};

#[derive(Bundle, Clone)]
pub struct Creature {
    pub(crate) starting_pos: Pos,
    pub(crate) genome: CreatureGenome,
    pub(crate) color: CodekaryoteColor,
    pub(crate) body: CodekaryoteBody,
    pub(crate) movement: Movement,
    pub(crate) eyes: Eyes,
    pub(crate) energy_storage: EnergyStorage,
    pub(crate) brain: Brain,
    #[bundle]
    pub mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
}

impl Creature {
    pub fn new(genome: CreatureGenome, pos: Pos, param: CodekaryoteParameters) -> Self {
        let mut c = Creature {
            starting_pos: pos,
            genome: genome.clone(),
            color: CodekaryoteColor::new(genome.color, param),
            body: CodekaryoteBody::new(genome.body, param),
            movement: Movement::new(genome.movement, param),
            eyes: Eyes::new(genome.eyes, param),
            energy_storage: EnergyStorage::new(genome.energy_storage, param),
            brain: Brain::new(genome.brain, param),
            mesh_bundle: default(),
        };

        c.energy_storage.init(c.body.clone());

        c
    }

    pub fn new_rand(limits: (f32, f32), param: CodekaryoteParameters) -> Self {
        Self::new(CreatureGenome::new(), Pos::rand(limits), param)
    }

    pub fn create_mesh(&self) -> (Circle, ColorMaterial) {
        let color = Color::rgb(self.color.r, self.color.g, self.color.b);
        let circle = Circle::new(self.body.size);
        let material = ColorMaterial::from(color);
        (circle, material)
    }

    pub fn create_body(
        &self,
    ) -> (
        RigidBody,
        Collider,
        ExternalForce,
        Velocity,
        Damping,
        ActiveEvents,
    ) {
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
            ActiveEvents::COLLISION_EVENTS,
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
        let coll = Collider::convex_polyline(vertex).unwrap();
        coll
    }
}

pub fn spawn_creature(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    mut creature: Creature,
) {
    let mesh_param = creature.create_mesh();
    let body_param = creature.create_body();
    let eyes_collider = creature.create_eye_sensors();

    creature.mesh_bundle = MaterialMesh2dBundle {
        mesh: meshes.add(mesh_param.0.into()).into(),
        material: materials.add(mesh_param.1),
        transform: Transform::from_translation(Vec3::new(
            creature.starting_pos.x,
            creature.starting_pos.y,
            0.,
        )),
        ..default()
    };

    let creature_entity = commands
        .spawn_bundle(creature)
        .insert(Kind::Creature)
        .insert(body_param.0)
        .insert(body_param.1)
        .insert(body_param.2)
        .insert(body_param.3)
        .insert(body_param.4)
        .insert(body_param.5)
        .id();

    let joint = FixedJointBuilder::new().local_anchor1(Vec2::new(0.0, 0.0));
    let eyes_entity = [commands
        .spawn()
        .insert(ImpulseJoint::new(creature_entity, joint))
        //.insert(RigidBody::Dynamic)
        .insert(eyes_collider)
        .insert(ColliderMassProperties::Mass(0.0))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Parent {
            entity: creature_entity,
        })
        .id()];

    commands
        .entity(creature_entity)
        .insert_children(0, &eyes_entity);
}
