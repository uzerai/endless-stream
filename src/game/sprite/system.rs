use bevy::prelude::*;
use crate::game::movement::component::Movable;
use crate::game::sprite::component::{AnimationTimer, FacingDirection, Layered};

//Handles the animation switching for Movable, AnimationTimer TextureAtlas sprites.
// TODO: move the actual animation component out of this so it will still apply to static animated sprites; separate as "sprite animation system"
pub fn animate_sprite(
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
// Updates the transform.translation.y coordinate to a float between 0 and 1,
// TODO: need to genericize to accept the current level size later (hardcoded to 2000).
pub fn layering_system(
    mut sprite_entities: Query<&mut Transform, (With<Movable>, With<Layered>)>,
) {
     for mut transform in &mut sprite_entities {
        let next_translation = 0.5 - (transform.translation.y / 2000.);

        transform.translation.z = next_translation;

        // info!("layering: <{},{},{}>", transform.translation.x, transform.translation.y, transform.translation.z);
     }
}