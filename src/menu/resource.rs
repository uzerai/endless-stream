use bevy::prelude::*;

#[derive(Resource)]
pub struct MenuButtons {
    pub hover_me_button: Entity,
}

#[derive(Resource)]
pub struct MenuCamera {
    pub camera: Entity,
}