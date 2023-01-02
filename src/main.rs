use bevy::prelude::*;
use endless_stream::AppState;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest())
        )
        .add_state(AppState::MainMenu)
        .add_plugin(endless_stream::game::game_plugin::GamePlugin)
        .add_plugin(endless_stream::menu::menu_plugin::MenuPlugin)
        .run();
}
