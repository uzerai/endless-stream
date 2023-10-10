use bevy::prelude::*;
use menu::MenuPlugin;
use game::GamePlugin;

pub mod menu;
pub mod game;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    SplashScreen,
    Loading,
    GamePlaying,
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(MenuPlugin)
        .add_plugins(GamePlugin)
        .run();
}
