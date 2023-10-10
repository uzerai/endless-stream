use bevy::prelude::*;

// To allow facing Sprite entities in either X direction.
#[derive(Component)]
pub enum FacingDirection {
    East,
    West
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct Layered;