use crate::life::codekaryotes::Kind;
use crate::life::common_parts::{CodekaryoteBody, Parent, MASS_ENERGY, MASS_ENERGY_RATE};
use crate::life::creature_parts::{EnergyStorage, Eyes, Seen};
use crate::KeyCode::Return;
use crate::{Entity, EventReader, Mut, Query, Transform, World};
use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use std::error::Error;

pub fn collision_event_dispatcher(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut eyes_query: Query<&mut Eyes>,
    mut energy_query: Query<&mut EnergyStorage>,
    sensor_query: Query<&Parent>,
    body_query: Query<&CodekaryoteBody>,
    transform_query: Query<&Transform>,
    kind_query: Query<&Kind>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(en1, en2, CollisionEventFlags::SENSOR) => {
                let parent_o = get_sensor_parent(&sensor_query, en1);

                let mut eyes = match parent_o {
                    Some(parent) => {
                        let mut eyes_r = eyes_query.get_mut(parent);
                        match eyes_r {
                            Ok(mut e) => e,
                            Err(e) => {
                                continue;
                            }
                        }
                    }
                    None => continue,
                };

                let pos1 = match transform_query.get(*en1) {
                    Ok(pos) => pos.translation,
                    Err(_) => {
                        continue;
                    }
                };

                let pos2 = match transform_query.get(*en2) {
                    Ok(pos) => pos.translation,
                    Err(_) => {
                        continue;
                    }
                };

                let other = match body_query.get(*en2) {
                    Ok(val) => val,
                    Err(_) => {
                        continue;
                    }
                };

                let seen = Seen::new(pos1, pos2, other.size);
                match kind_query.get(*en2) {
                    Ok(k) => match k {
                        Kind::Creature => {
                            eyes.seen_creature.insert(en2.id(), seen);
                        }
                        Kind::Plant => {
                            eyes.seen_plants.insert(en2.id(), seen);
                        }
                    },
                    Err(_) => {}
                }
            }
            CollisionEvent::Stopped(en1, en2, CollisionEventFlags::SENSOR) => {
                let parent_o = get_sensor_parent(&sensor_query, en1);

                let mut eyes = match parent_o {
                    Some(parent) => {
                        let mut eyes_r = eyes_query.get_mut(parent);
                        match eyes_r {
                            Ok(mut e) => e,
                            Err(e) => {
                                println!("{}", e);
                                continue;
                            }
                        }
                    }
                    None => continue,
                };

                match kind_query.get(*en2) {
                    Ok(k) => match k {
                        Kind::Creature => {
                            eyes.seen_creature.remove(&en2.id());
                        }
                        Kind::Plant => {
                            eyes.seen_plants.remove(&en2.id());
                        }
                    },
                    Err(_) => {}
                }
            }
            CollisionEvent::Started(en1, en2, e) => {
                match kind_query.get(*en2) {
                    Ok(k) => match k {
                        Kind::Creature => {
                            // do nothing for now, this is where predation will go
                        }
                        Kind::Plant => {
                            //get the bodies
                            let own_body = match body_query.get(*en1) {
                                Ok(val) => val,
                                Err(_) => {
                                    continue;
                                }
                            };

                            let other_body = match body_query.get(*en2) {
                                Ok(val) => val,
                                Err(_) => {
                                    continue;
                                }
                            };

                            if own_body.size > other_body.size {
                                //eat the plant

                                let energy = other_body.mass * MASS_ENERGY;
                                let mut energy_storage = match energy_query.get_mut(*en1) {
                                    Ok(mut val) => val,
                                    Err(_) => {
                                        continue;
                                    }
                                };
                                energy_storage.add_energy(energy);
                                commands.entity(*en2).despawn_recursive();
                            }
                        }
                    },
                    Err(_) => {}
                }
            }
            CollisionEvent::Stopped(en1, en2, e) => {}
        }
    }
}

fn get_sensor_parent(sensor_query: &Query<&Parent>, en1: &Entity) -> Option<Entity> {
    let sensor_r = sensor_query.get(*en1);

    match sensor_r {
        Ok(sensor) => Some(sensor.entity),
        Err(e) => None,
    }
}
