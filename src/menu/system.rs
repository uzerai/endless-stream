use bevy::prelude::*;
use super::{ UNHOVERED_BUTTON_COLOR, HOVERED_BUTTON_COLOR };
use super::component::MenuButtonAction;
use crate::AppState;

pub fn menu_button_feel_system(
    // mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::Pressed => {
                // do nothing
            }
            Interaction::None => {
                *color = UNHOVERED_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn menu_action_system(
    interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Play => {
                    game_state.set(AppState::GamePlaying);
                    println!("PLAY BUTTON PRESSED")
                },
                _ => {}
            }
        }
    }
}