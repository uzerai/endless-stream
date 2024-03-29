use bevy::prelude::*;
use super::component::PlayerHealthIndicator;
use crate::game::player::component::PlayerControlled;
use crate::game::health::component::Health;

pub fn player_health_indicator_update(
    mut player_health_indicator: Query<&mut Sprite, With<PlayerHealthIndicator>>,
    player_health: Query<&Health, With<PlayerControlled>>,
) {
    let player_health = player_health.single();
    // info!("Player health: {}", player_health.current);

    for mut health_sprite in &mut player_health_indicator {
        let health_decimal = player_health.current / player_health.max;
        health_sprite.custom_size = Some(Vec2::new(20. * health_decimal, 5.))
    }
}