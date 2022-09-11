use crate::life::codekaryotes::Kind;
use crate::life::common_parts::{CodekaryoteBody, Parent};
use crate::life::creature_parts::{Eyes, Seen};
use crate::KeyCode::Return;
use crate::{Entity, EventReader, Mut, Query, Transform, World};
use bevy::ecs::query::QueryEntityError;
use bevy_rapier2d::pipeline::CollisionEvent;
use bevy_rapier2d::prelude::Sensor;
use bevy_rapier2d::rapier::geometry::CollisionEventFlags;
use bevy_rapier2d::rapier::prelude::BodyPair;
use std::error::Error;

pub fn collision_event_dispatcher(
    mut collision_events: EventReader<CollisionEvent>,
    mut eyes_query: Query<&mut Eyes>,
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
            _ => todo!("That's a new type of contact you need to deal with"),
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
