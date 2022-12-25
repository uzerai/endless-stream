use bevy::log::*;
use bevy::app::Plugin;
use bevy::app::App;
use bevy::ecs::query::With;
use bevy::ecs::query::Without;
use bevy::ecs::component::Component;
use bevy::sprite::Sprite;
use bevy::transform::components::Transform;
use bevy::time::Time;
use bevy::ecs::system::Res;
use bevy::ecs::system::Query;
use bevy::math::Vec2;
use bevy::sprite::collide_aabb::{ collide, Collision };

pub struct EntityMovementPlugin;

impl Plugin for EntityMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement_system);
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
    pub size: Transform,
}

// TODO: Need to figure out a way to anchor the collidable to the bottom of each sprite, 
// rather than in the center and expanding outwards from there.
// this is required to support uneven shapes.
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

fn movement_system(
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