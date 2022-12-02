use bevy::prelude::*;
use bevy::sprite::collide_aabb::{ collide, Collision };

pub struct PlayerControlledEntityPlugin;    

impl Plugin for PlayerControlledEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(collision_system)
            .add_system(layering_system)
            .add_system(keyboard_input_system)
            .add_system(animate_sprite);
    }
}

#[derive(Component)]
struct LevelFloor;

#[derive(Component)]
struct PlayerControlled;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Movable {
    velocity: f32,
    direction: Vec2
}

#[derive(Component)]
struct Collidable {
    size: Transform
}

impl Collidable {
    fn size(self: &Collidable) -> Vec2 {
        return self.size.scale.truncate();
    }
}

// To allow facing Sprite entities in either X direction.
#[derive(Component)]
enum FacingDirection {
    East,
    West
}

impl Movable {
    fn get_x_direction(self: &Movable) -> f32 {
        Vec2::as_ref(&Vec2::normalize_or_zero(self.direction))[0]
    }

    fn get_y_direction(self: &Movable) -> f32 {
        Vec2::as_ref(&Vec2::normalize_or_zero(self.direction))[1]
    }
}

// Handles keyboard events for any PlayerControlled Component-initializes entities.
// TODO: allow for changing keybinds -- will come with the menu system me thinks
fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut player_character: Query<(&mut Movable, &PlayerControlled)>) {
    for (mut movable, _) in &mut player_character {
        if keyboard_input.pressed(KeyCode::A) {
            movable.direction += Vec2::new(-0.3, 0.)
        }

        if keyboard_input.pressed(KeyCode::S) {
            movable.direction += Vec2::new(0., -0.3)
        }

        if keyboard_input.pressed(KeyCode::W) {
            movable.direction += Vec2::new(0., 0.3)
        }

        if keyboard_input.pressed(KeyCode::D) {
            movable.direction += Vec2::new(0.3, 0.)
        }

        if keyboard_input.any_just_released([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D]) {
            movable.direction = Vec2::ZERO
        }
    }
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

// Handles the animation switching for Movable, AnimationTimer TextureAtlas sprites.
// TODO: move the actual animation component out of this so it will still apply to static animated sprites; separate as "sprite animation system"
fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut movable_sprite_entity: Query<(
        &mut Movable,
        &mut FacingDirection,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (movable_struct, mut facing_direction, mut timer, mut sprite, texture_atlas_handle) in &mut movable_sprite_entity {
        timer.tick(time.delta());
        if timer.just_finished() {
            if movable_struct.get_x_direction().gt(&0.) {
                *facing_direction = FacingDirection::East
            }

            if movable_struct.get_x_direction().lt(&0.) {
                *facing_direction = FacingDirection::West
            }

            if movable_struct.get_x_direction() == 0. && movable_struct.get_y_direction() == 0. {
                // If the sprite is _not_ moving; use only the first frame of the loop, don't bother
                // loading the texture atlas as it'll save on individual frame times.
                sprite.index = 0;
            } else {
                let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                // if the character is moving in either X direction; flip the sprite as necessary.

                match  *facing_direction {
                    FacingDirection::East => sprite.flip_x = false,
                    FacingDirection::West => sprite.flip_x = true,
                }
            }
        }
    }
}

// Ensures sprite layering when moving up/down the level
fn layering_system(
    mut sprite_entities: Query<(&mut Transform, &Movable), Without<Camera2d>>,
) {
     for (mut transform, _) in &mut sprite_entities {
        let next_layer = transform.translation.y * -0.01;
        if next_layer > 0. && next_layer < 999.{
            transform.translation.z = transform.translation.y * -0.1;
        }
        info!("layering: <{},{},{}>", transform.translation.x, transform.translation.y, transform.translation.z);
     }
}

// Handles collision with level edges and other collidable entities
fn collision_system(
    time: Res<Time>, 
    mut player_entity: Query<(&Movable, &mut Transform, &Collidable, &PlayerControlled)>, 
    mut other_entities: Query<(&Transform, &Collidable), (Without<PlayerControlled>, Without<Movable>)>,
    level: Query<(&Transform, &Sprite), (With<LevelFloor>, Without<Collidable>)>
) {
    let (level_transform, level_sprite) = level.single();

     for (player_movable_struct,  mut player_transform, player_collidable, _) in &mut player_entity {
        let mut y_move = player_movable_struct.get_y_direction() * player_movable_struct.velocity * time.delta_seconds();
        let mut x_move = player_movable_struct.get_x_direction() * player_movable_struct.velocity * time.delta_seconds();

        let mut next_translation = player_transform.translation.clone();
        next_translation.x += x_move;
        next_translation.y += y_move;

        for (collidable_entity_transform, collidable_entity_collidable) in &mut other_entities {
            let mut collidable_translation = collidable_entity_transform.translation.clone();
            // We always assume they'll collide in a 2d plane.
            collidable_translation.z = 1.;
            next_translation.z = 1.;

            match collide(next_translation, player_collidable.size(), collidable_translation, collidable_entity_collidable.size()) {
                None => {},
                Some(collision) => {
                    match collision {
                        Collision::Top => { if y_move.lt(&0.) { y_move = 0. } },
                        Collision::Bottom => { if y_move.gt(&0.) { y_move = 0. } },
                        Collision::Left => { if x_move.gt(&0.) { x_move = 0. } },
                        Collision::Right => { if x_move.lt(&0.) { x_move = 0. } },
                        Collision::Inside => {
                            x_move = 0.;
                            y_move= 0.;
                        }
                    }
                }
            }

             match collide(next_translation, player_collidable.size(), level_transform.translation, level_sprite.custom_size.expect("REASON")) {
                None => {
                    warn!("No collision with level.");
                },
                Some(collision) => {
                    match collision {
                        Collision::Inside => {
                            player_transform.translation.y += y_move;
                            player_transform.translation.x += x_move;
                        },
                        _ => {}
                    }
                }
            }

            // info!("position: <{},{},{}>", player_transform.translation.x, player_transform.translation.y, player_transform.translation.z);
        }
    }
}
