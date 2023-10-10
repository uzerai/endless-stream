use bevy::prelude::*;
use component::MenuButtonAction;
use resource::MenuButtons;
use crate::AppState;
use system::*;
use crate::menu::resource::MenuCamera;

pub mod component;
pub mod system;
pub mod resource;

const UNHOVERED_BUTTON_COLOR: Color = Color::rgb(1., 0., 0.);
const HOVERED_BUTTON_COLOR: Color = Color::rgb(0., 0., 1.);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MenuState {
    #[default]
    MainMenu,
    PauseMenu,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), (setup_menu, setup_2d_camera))
            .add_systems(Update, menu_button_feel_system.run_if(in_state(AppState::MainMenu)))
            .add_systems(Update,menu_action_system.run_if(in_state(AppState::MainMenu)))
            .add_systems(OnExit(AppState::MainMenu), cleanup_menu);
    }
}

pub fn setup_menu(mut commands: Commands) {
    let hover_me_button = commands
        .spawn(NodeBundle {
            // Centered column for buttons to go into.
            style: Style {
                width: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }).with_children(|parent| {
        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Percent(40.),
                height: Val::Px(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: UNHOVERED_BUTTON_COLOR.into(),
            ..default()
        }, MenuButtonAction::Play)).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Play",
                TextStyle {
                    font_size: 40.,
                    color: Color::rgb(1., 1., 1.),
                    ..default()
                },
            ));
        });
    })
        .id();

    commands.insert_resource(MenuButtons { hover_me_button })
}

fn setup_2d_camera(mut commands: Commands) {
    let camera = commands.spawn(Camera2dBundle::default()).id();
    commands.insert_resource(MenuCamera { camera });
}

pub fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuButtons>, camera: Res<MenuCamera>) {
    commands.entity(menu_data.hover_me_button).despawn_recursive();
    commands.entity(camera.camera).despawn_recursive();
}