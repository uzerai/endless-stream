use bevy::prelude::*;

use crate::game::movement::component::Movable;
use crate::game::health::component::Health;
use crate::game::player::component::PlayerControlled;

// Handles keyboard events for any PlayerControlled Component-initializes entities.
// TODO: allow for changing keybinds -- will come with the menu system me thinks
pub fn keyboard_input_system(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut player_character: Query<(&mut Movable, &mut Health), With<PlayerControlled>>,
    // mut app_state: ResMut<NextState<AppState>>,
) {
    for (mut movable, mut health) in &mut player_character {
        if keyboard_input.pressed(KeyCode::A) {
            movable.direction += Vec2::new(-0.3, 0.)
        }

        if keyboard_input.pressed(KeyCode::S) {
            movable.direction += Vec2::new(0., -0.5)
        }

        if keyboard_input.pressed(KeyCode::W) {
            movable.direction += Vec2::new(0., 0.5)
        }

        if keyboard_input.pressed(KeyCode::D) {
            movable.direction += Vec2::new(0.5, 0.)
        }

        //TODO: remove after health testing.
        if keyboard_input.pressed(KeyCode::U) {
            health.current -= 1.;
            info!("Removing 1 hp");
        }

        // if keyboard_input.pressed(KeyCode::Space) {
        //     app_state.set(AppState::MainMenu);
        //     keyboard_input.reset(KeyCode::Space);
        // }

        if keyboard_input.any_just_released([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D]) {
            movable.direction = Vec2::ZERO
        }
    }
}