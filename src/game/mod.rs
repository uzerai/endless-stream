use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::AppState;
use player::system::keyboard_input_system;
use sprite::system::{ animate_sprite, layering_system };
use movement::system::entity_movement_system;
use level::component::LevelFloor;
use crate::game::health::component::{Health, PlayerHealthIndicator};
use crate::game::movement::component::{Collidable, Movable};
use crate::game::player::component::PlayerControlled;
use crate::game::sprite::component::{AnimationTimer, FacingDirection, Layered};

pub mod game_state;
pub mod level;
pub mod movement;
pub mod player;
pub mod sprite;
pub mod health;



#[derive(Component)]
pub struct GameEntity;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GamePlaying), game_setup)
            .add_systems(Update, (animate_sprite, layering_system).run_if(in_state(AppState::GamePlaying)))
            .add_systems(Update, (entity_movement_system).run_if(in_state(AppState::GamePlaying)))
            .add_systems(Update, keyboard_input_system.run_if(in_state(AppState::GamePlaying)))
            .add_systems(OnExit(AppState::MainMenu), despawn_screen::<GameEntity>);
    }
}

fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("player_character/gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Spawn and insert the background for the "walkable" level
    commands.spawn(
        (
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0., 128., 0.),
                    custom_size: Some(Vec2::new(5000., 2000.)),
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            },
            GameEntity,
            LevelFloor
        )
    );

    // Let's just spawn a sine-wave of trees.
    for x_spawn_coord in -2500i32..2501i32 {
        if x_spawn_coord.abs() % 50 == 0 {
            let x_spawn_float = x_spawn_coord as f32;
            let y_spawn_float = x_spawn_float.cos() * 500.;
            let z_spawn_float = 0.5 - (y_spawn_float / 2000.);
            spawn_tree_at(&mut commands, &asset_server, Vec3::new(x_spawn_float, y_spawn_float, z_spawn_float));

            // info!("Spawning tree at: <{},{},{}>", x_spawn_float, y_spawn_float, z_spawn_float);
        }
    }

    let circle_spawn_radius = 700.;
    let angle_interval = 15f32;

    for angle in 1i32..360i32 {
        let angle_float = angle as f32;
        if angle_float % angle_interval == 0. {
            let x_spawn_float = circle_spawn_radius * angle_float.cos();
            let y_spawn_float = circle_spawn_radius * angle_float.sin();

            spawn_enemy_at(&mut commands, &asset_server, &mut texture_atlases, Vec3::new(x_spawn_float, y_spawn_float, 0.));
        }
    }

    // spawn_enemy_at(&mut commands, &asset_server, &mut texture_atlases, Vec3::new(-400., 50., 0.));

    commands.spawn(
        (
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_scale(Vec3::splat(2.)),
                ..default()
            },
            GameEntity,
            PlayerControlled,
            Movable {
                velocity: 200.,
                direction: Vec2::ZERO,
            },
            Collidable {
                size: Transform::from_scale(Vec3::new(26., 20., 1.))
            },
            FacingDirection::East,
            Layered,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Health {
                max: 100.,
                current: 100.
            }
        )
    ).with_children(|parent| {
        parent.spawn(
            (
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(1., 0., 0.),
                        custom_size: Some(Vec2::new(20., 5.)),
                        anchor: Anchor::TopLeft,
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(-10., -20., 0.)),
                    ..default()
                },
                GameEntity,
                PlayerHealthIndicator
            )
        ).with_children(|parent| {
            parent.spawn(
                (
                    SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0., 0., 0.),
                        custom_size: Some(Vec2::new(20., 5.)),
                        anchor: Anchor::TopLeft,
                        ..default()
                    },
                    transform: Transform::from_xyz(0., 0., -0.1),
                        ..default()
                    },
                    GameEntity,
                )
            );
        });

        parent.spawn(
            (
                Camera2dBundle {
                    transform: Transform::from_translation(Vec3::new(0., 0., 999.)),
                    projection: OrthographicProjection {
                        scale: 0.5,
                        ..default()
                    },
                    ..default()
                },
                GameEntity,
            )
        );
    });
}

fn spawn_tree_at(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    vec3_translation: Vec3,
) {
    let tree_texture_handle = asset_server.load("environment/tree-sprite.png");

    commands.spawn(
        (
            SpriteBundle {
                texture: tree_texture_handle,
                sprite: Sprite {
                    anchor: Anchor::Custom(Vec2::new(0., -0.25)),
                    custom_size: Some(Vec2::new(60., 60.)),
                    ..default()
                },
                transform: Transform::from_scale(Vec3::splat(2.))
                    .with_translation(vec3_translation),
                ..default()
            },
            GameEntity,
            Collidable {
                size: Transform::from_scale(Vec3::new(16., 20., 1.))
            },
            FacingDirection::East,
        )
    );
}
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

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
    let texture_handle = asset_server.load("player_character/gabe-idle-run.png");
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
            GameEntity,
            FacingDirection::East,
            Layered,
            Enemy,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
        )
    );
}