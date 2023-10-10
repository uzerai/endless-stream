use bevy::prelude::*;
use bevy::math::Vec2;

#[derive(Component)]
pub struct Collidable {
    pub size: Transform,
}

// TODO: Need to figure out a way to anchor the collidable to the bottom of each sprite,
// rather than in the center and expanding outwards from there.
// this is required to support uneven shapes.
impl Collidable {
    pub(crate) fn size(self: &Collidable) -> Vec2 {
        return self.size.scale.truncate();
    }
}

#[derive(Component)]
pub struct Movable {
    pub velocity: f32,
    pub direction: Vec2
}

impl Movable {
    pub fn get_x_direction(self: &Movable) -> f32 {
        Vec2::as_ref(&Vec2::normalize_or_zero(self.direction))[0]
    }

    pub fn get_y_direction(self: &Movable) -> f32 {
        Vec2::as_ref(&Vec2::normalize_or_zero(self.direction))[1]
    }
}