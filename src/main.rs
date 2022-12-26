use bevy::prelude::*;
use bevy::sprite::Anchor;

use endless_stream::player_control::{ 
    Layered, 
    FacingDirection, 
    AnimationTimer, 
    PlayerControlled, 
    PlayerControlPlugin 
};
use endless_stream::movement::{
    LevelFloor,
    Collidable,
    Movable,
    EntityMovementPlugin
};
use endless_stream::entity_health::{ Health, PlayerHealthIndicator, EntityHealthPlugin };
use endless_stream::enemy::{ regular_enemy_movement, spawn_enemy_at };

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest())
        )
        .add_plugin(PlayerControlPlugin)
        .add_plugin(EntityMovementPlugin)
        .add_plugin(EntityHealthPlugin)
        .add_system(regular_enemy_movement)
        .run();
}


fn setup(
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
    let angle_interval = 5f32;
    
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
                PlayerHealthIndicator
            )
        ).with_children(|parent| {
            parent.spawn(
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0., 0., 0.),
                        custom_size: Some(Vec2::new(20., 5.)),
                        anchor: Anchor::TopLeft,
                        ..default()
                    },
                    transform: Transform::from_xyz(0., 0., -0.1),
                    ..default()
                }
            );
        });

        parent.spawn(
            Camera2dBundle {
                transform: Transform::from_translation(Vec3::new(0., 0., 999.)),
                projection: OrthographicProjection {
                    scale: 0.5,
                    ..default()
                },
                ..default()
            }
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
            Collidable {
                size: Transform::from_scale(Vec3::new(16., 20., 1.))
            },
            FacingDirection::East,
        )
    );
}
