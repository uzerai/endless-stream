use bevy::math::Vec2;
use crate::player_control::PlayerControlled;
use bevy::ecs::query::With;
use bevy::sprite::Sprite;
use bevy::ecs::system::Query;
use bevy::app::App;
use bevy::app::Plugin;
use bevy::ecs::component::Component;
use bevy::log::info;


pub struct EntityHealthPlugin;

impl Plugin for EntityHealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_health_indicator_update);
    }
}

fn player_health_indicator_update(
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

#[derive(Component)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}

#[derive(Component)]
pub struct PlayerHealthIndicator;