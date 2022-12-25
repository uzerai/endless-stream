use endless_stream::movement::LevelFloor;
use endless_stream::player_control::Layered;
use endless_stream::player_control::FacingDirection;
use endless_stream::player_control::AnimationTimer;
use endless_stream::movement::Collidable;
use endless_stream::player_control::PlayerControlled;
use endless_stream::movement::Movable;
use endless_stream::player_control::PlayerControlPlugin;
use endless_stream::movement::EntityMovementPlugin;

use bevy::prelude::*;
use bevy::sprite::Anchor;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(PlayerControlPlugin)
        .add_plugin(EntityMovementPlugin)
        .run();
}


fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let texture_handle = asset_server.load("gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Spawn and set the camera
    commands.spawn(
        (
            Camera2dBundle::default(), Movable {
                velocity: 200.,
                direction: Vec2::ZERO,
            }, 
            Collidable {
                size: Transform::from_scale(Vec3::new(26., 20., 3.))
            },
            PlayerControlled
        )
    );

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
        if x_spawn_coord.abs() % 100 == 0 {
            let x_spawn_float = x_spawn_coord as f32;
            let y_spawn_float = x_spawn_float.sin() * 500.;
            let z_spawn_float = 0.5 - (y_spawn_float / 2000.);
            spawn_tree_at(&mut commands, &asset_server, Vec3::new(x_spawn_float, y_spawn_float, z_spawn_float));

            // info!("Spawning tree at: <{},{},{}>", x_spawn_float, y_spawn_float, z_spawn_float);
        }
    }

    // This creates the player character.
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
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
        )
    );
}

fn spawn_tree_at(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>,
    vec3_translation: Vec3,
) {
    let tree_texture_handle = asset_server.load("tree-sprite.png");

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
