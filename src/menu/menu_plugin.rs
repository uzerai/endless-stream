use bevy::ecs::query::With;
use bevy::prelude::DespawnRecursiveExt;
use bevy::ecs::system::Query;
use bevy::ecs::entity::Entity;
use bevy::ui::node_bundles::TextBundle;
use bevy::prelude::BuildChildren;
use bevy::ui::node_bundles::NodeBundle;
use bevy::text::TextStyle;
use bevy::ui::UiRect;
use bevy::ui::Style;
use bevy::ui::Size;
use bevy::ui::Val;
use bevy::ui::FlexDirection;
use bevy::ui::JustifyContent;
use bevy::ui::AlignItems;
use bevy::math::Vec3;
use bevy::transform::components::Transform;
use bevy::render::camera::OrthographicProjection;
use bevy::utils::default;
use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::asset::AssetServer;
use bevy::ecs::system::Res;
use bevy::ecs::system::Commands;
use bevy::render::color::Color;
use bevy::input::Input;
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
pub enum MenuButtonAction {
    Play,
}

#[derive(Component)]
pub struct MenuEntity;

impl Plugin for MenuPlugin {
	 fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
                .with_system(menu_setup)
        )
        .add_system_set(
            SystemSet::on_update(AppState::MainMenu)
            	.with_system(keyboard_input_system)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::MainMenu)
                .with_system(despawn_screen::<MenuEntity>)
        );
    }
}

const MENU_TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let font = asset_server.load("font/Golden Age.ttf");

    // create a 2d camera;
    commands.spawn(
        (
            Camera2dBundle {
                transform: Transform::from_translation(Vec3::new(0., 0., 999.)),
                projection: OrthographicProjection {
                    scale: 0.5,
                    ..default()
                },
                ..default()
            },
            MenuEntity,
        )
    );

    let button_style = Style {
        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: MENU_TEXT_COLOR,
    };

    commands.spawn((
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        MenuEntity
    )).with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::CRIMSON.into(),
            ..default()
        }).with_children(|parent| {
            // Display the game name
            parent.spawn(
                TextBundle::from_section(
                    "Press Space",
                    TextStyle {
                        font: font.clone(),
                        font_size: 80.0,
                        color: MENU_TEXT_COLOR,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );
        });
    });
}

pub fn keyboard_input_system(
    mut keyboard_input: ResMut<Input<KeyCode>>, 
    mut app_state: ResMut<State<AppState>>,
) {
        if keyboard_input.pressed(KeyCode::Space) {
            app_state.set(AppState::InGame).unwrap();
            keyboard_input.reset(KeyCode::Space);
        }
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}