use bevy::prelude::*;
use crate::movement::Movable;

pub struct PlayerControlPlugin;

impl Plugin for PlayerControlPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(layering_system)
            .add_system(keyboard_input_system)
            .add_system(animate_sprite);
    }
}

#[derive(Component)]
pub struct PlayerControlled;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

// To allow facing Sprite entities in either X direction.
#[derive(Component)]
pub enum FacingDirection {
    East,
    West
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
