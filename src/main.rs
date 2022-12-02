use bevy::prelude::*;
use endless_stream::player_controlled_entity::PlayerControlledEntityPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(PlayerControlledEntityPlugin)
        .run();
}

