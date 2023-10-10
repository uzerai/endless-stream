use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}

#[derive(Component)]
pub struct PlayerHealthIndicator;