use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use crate::game::level::component::LevelFloor;
use crate::game::movement::component::{ Collidable, Movable };

pub fn entity_movement_system(
    time: Res<Time>,
    mut movable_entities: Query<(&Movable, &mut Transform, &Collidable)>,
    static_entities: Query<(&Transform, &Collidable), (Without<Movable>, Without<LevelFloor>)>,
    level: Query<(&Transform, &Sprite), (With<LevelFloor>, Without<Collidable>)>
) {
    let (level_transform, level_sprite) = level.single();

    for(movable, mut transform, collidable) in &mut movable_entities {
        let translation = transform.translation;

        // info!("movable entity position at: <{},{},{}>", translation.x, translation.y, translation.z);
        // info!("movable entity movable direction: <{},{}>", movable.get_x_direction(), movable.get_y_direction());
        let mut y_move = movable.get_y_direction() * movable.velocity * time.delta_seconds();
        let mut x_move = movable.get_x_direction() * movable.velocity * time.delta_seconds();

        let mut next_translation = translation.clone();
        next_translation.x += x_move;
        next_translation.y += y_move;

        for (static_entity_transform, static_entity_collidable) in &static_entities {
            let mut static_entity_translation = static_entity_transform.translation.clone();

            // bevy's collision works in 3d, let's make sure the translations are in the same z axis.
            next_translation.z = 1.0;
            static_entity_translation.z = 1.0;

            match collide(next_translation, collidable.size(), static_entity_translation, static_entity_collidable.size()) {
                None => {},
                Some(collision) => {
                    match collision {
                        Collision::Top => { if y_move.lt(&0.) { y_move = 0. } },
                        Collision::Bottom => { if y_move.gt(&0.) { y_move = 0. } },
                        Collision::Left => { if x_move.gt(&0.) { x_move = 0. } },
                        Collision::Right => { if x_move.lt(&0.) { x_move = 0. } },
                        Collision::Inside => {
                            x_move = 0.;
                            y_move= 0.;
                        }
                    }
                }
            }
        }

        match collide(next_translation, collidable.size(), level_transform.translation, level_sprite.custom_size.expect("No levelsprite custom size; assuming no level loaded.")) {
            None => {
                info!("No collision with level.");
            },
            Some(collision) => {
                match collision {
                    Collision::Inside => {
                        transform.translation.x += x_move;
                        transform.translation.y += y_move;
                    },
                    _ => {}
                }
            }
        }
    }
}