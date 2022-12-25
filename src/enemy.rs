use bevy::time::Timer;
use crate::player_control::AnimationTimer;
use crate::player_control::Layered;
use bevy::utils::default;
use bevy::sprite::SpriteSheetBundle;
use bevy::math::Vec2;
use bevy::math::Vec3;
use bevy::sprite::TextureAtlas;
use bevy::asset::Assets;
use bevy::ecs::system::ResMut;
use bevy::asset::AssetServer;
use bevy::ecs::system::Res;
use bevy::ecs::system::Commands;
use bevy::core_pipeline::core_2d::Camera2d;
use bevy::ecs::query::Without;
use bevy::ecs::query::With;
use bevy::transform::components::Transform;
use bevy::ecs::system::Query;
use bevy::ecs::component::Component;
use bevy::prelude::TimerMode;

use crate::movement::{ Movable, Collidable };
use crate::player_control::{ PlayerControlled,  FacingDirection };


// Tag interface for all enemies.
#[derive(Component)]
pub struct Enemy;

// Tag interface for marking enemies that will not follow the regular movement pattern of always pathing towards the player.
#[derive(Component)]
pub struct IrregularEnemy;

// Makes use of the Movable struct to point the enemy to always walk towards the player character.
pub fn regular_enemy_movement(
    player_character: Query<(
        &Movable, &mut Transform, &Collidable), 
        (With<PlayerControlled>, (Without<Camera2d>, Without<Enemy>)
    )>,
    mut enemies: Query<(&mut Movable, &mut Transform, &Collidable), (With<Enemy>, Without<IrregularEnemy>)>,
) {
    let (_player_movable, player_transform, _player_collide) = player_character.single();

    for (mut enemy_movable, enemy_transform, _) in &mut enemies {
        let target_dir = player_transform.translation.truncate() - enemy_transform.translation.truncate();
        enemy_movable.direction = target_dir;
    }
}

pub fn spawn_enemy_at(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    vec3_translation: Vec3
) {
    let texture_handle = asset_server.load("gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(
        (
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_scale(Vec3::splat(2.))
                    .with_translation(vec3_translation),
                ..default()
            },
            Movable {
                velocity: 150.,
                direction: Vec2::ZERO,
            },
            Collidable {
                size: Transform::from_scale(Vec3::new(26., 20., 1.))
            },
            FacingDirection::East,
            Layered,
            Enemy,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
        )
    );
}