use crate::player_control::PlayerControlled;

use bevy::log::*;
use bevy::app::Plugin;
use bevy::app::App;
use bevy::ecs::query::With;
use bevy::ecs::component::Component;
use bevy::sprite::Sprite;
use bevy::ecs::query::Without;
use bevy::transform::components::Transform;
use bevy::time::Time;
use bevy::ecs::system::Res;
use bevy::ecs::system::Query;
use bevy::math::Vec2;
use bevy::sprite::collide_aabb::{ collide, Collision };


pub struct EntityMovementPlugin;

impl Plugin for EntityMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collision_system);
    }
}

#[derive(Component)]
pub struct LevelFloor;

#[derive(Component)]
pub struct Movable {
    pub velocity: f32,
    pub direction: Vec2
}

#[derive(Component)]
pub struct Collidable {
    pub size: Transform
}

impl Collidable {
    fn size(self: &Collidable) -> Vec2 {
        return self.size.scale.truncate();
    }
}

impl Movable {
    pub fn get_x_direction(self: &Movable) -> f32 {
        Vec2::as_ref(&Vec2::normalize_or_zero(self.direction))[0]
    }

    pub fn get_y_direction(self: &Movable) -> f32 {
        Vec2::as_ref(&Vec2::normalize_or_zero(self.direction))[1]
    }
}

// Handles collision with level edges and other collidable entities
fn collision_system(
    time: Res<Time>, 
    mut player_entity: Query<(&Movable, &mut Transform, &Collidable, &PlayerControlled)>, 
    mut other_entities: Query<(&Transform, &Collidable), (Without<PlayerControlled>, Without<Movable>)>,
    level: Query<(&Transform, &Sprite), (With<LevelFloor>, Without<Collidable>)>
) {
    let (level_transform, level_sprite) = level.single();

     for (player_movable_struct,  mut player_transform, player_collidable, _) in &mut player_entity {
        let mut y_move = player_movable_struct.get_y_direction() * player_movable_struct.velocity * time.delta_seconds();
        let mut x_move = player_movable_struct.get_x_direction() * player_movable_struct.velocity * time.delta_seconds();

        let mut next_translation = player_transform.translation.clone();
        next_translation.x += x_move;
        next_translation.y += y_move;

        for (collidable_entity_transform, collidable_entity_collidable) in &mut other_entities {
            let mut collidable_translation = collidable_entity_transform.translation.clone();
            // We always assume they'll collide in a 2d plane.
            collidable_translation.z = 1.;
            next_translation.z = 1.;

            match collide(next_translation, player_collidable.size(), collidable_translation, collidable_entity_collidable.size()) {
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

             match collide(next_translation, player_collidable.size(), level_transform.translation, level_sprite.custom_size.expect("REASON")) {
                None => {
                    warn!("No collision with level.");
                },
                Some(collision) => {
                    match collision {
                        Collision::Inside => {
                            player_transform.translation.y += y_move;
                            player_transform.translation.x += x_move;
                        },
                        _ => {}
                    }
                }
            }

            // info!("position: <{},{},{}>", player_transform.translation.x, player_transform.translation.y, player_transform.translation.z);
        }
    }
}