use endless_stream::movement::LevelFloor;
use endless_stream::player_control::FacingDirection;
use endless_stream::player_control::AnimationTimer;
use endless_stream::movement::Collidable;
use endless_stream::player_control::PlayerControlled;
use endless_stream::movement::Movable;
use bevy::prelude::*;
use endless_stream::player_control::PlayerControlPlugin;
use endless_stream::movement::EntityMovementPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(PlayerControlPlugin)
        .add_plugin(EntityMovementPlugin)
        .add_startup_system(setup)
        .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let texture_handle = asset_server.load("gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let tree_texture_handle = asset_server.load("gabe-idle-run.png");
    let tree_texture_atlas = TextureAtlas::from_grid(tree_texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let tree_atlas_handle = texture_atlases.add(tree_texture_atlas);

    // Spawn and set the camera
    commands.spawn((Camera2dBundle::default(), Movable {
        velocity: 200.,
        direction: Vec2::ZERO,
    }, 
    Collidable {
        size: Transform::from_scale(Vec3::new(26., 40., 3.))
    },
     PlayerControlled));

    // Spawn and insert the background for the "walkable" level
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0., 128., 0.),
                custom_size: Some(Vec2::new(1000., 500.)),
                ..default()
            },
            ..default()
        }, LevelFloor)
    );

    commands.spawn(
        (
            SpriteSheetBundle {
                texture_atlas: tree_atlas_handle,
                transform: Transform::from_scale(Vec3::splat(2.))
                    .with_translation(Vec3::new(50.,0.,1.)),
                ..default()
            },
            Collidable {
                size: Transform::from_scale(Vec3::new(26., 5., 1.))
            },
            FacingDirection::East,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
        )
    );

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
                size: Transform::from_scale(Vec3::new(26., 40., 1.))
            },
            FacingDirection::East,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
        )
    );
}
