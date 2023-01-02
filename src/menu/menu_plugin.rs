use bevy::input::Input;
use bevy::ecs::system::Res;
use bevy::input::keyboard::KeyCode;
use bevy::ecs::system::ResMut;
use bevy::ecs::schedule::State;
use bevy::app::Plugin;
use bevy::app::App;
use bevy::ecs::schedule::SystemSet;
use bevy::ecs::component::Component;

use crate::AppState;

pub struct MenuPlugin;

#[derive(Component)]
pub struct MenuEntity;

impl Plugin for MenuPlugin {
	 fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
        )
        .add_system_set(
            SystemSet::on_update(AppState::MainMenu)
            	.with_system(keyboard_input_system)
        );
    }
}

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>, 
    mut app_state: ResMut<State<AppState>>,
) {
        if keyboard_input.pressed(KeyCode::Space) {
            app_state.set(AppState::InGame).unwrap();
        }
}