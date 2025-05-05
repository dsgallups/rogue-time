use bevy::{
    color::palettes::css::{BLACK, RED, WHITE},
    prelude::*,
};

use crate::{screens::Screen, theme::Containers};

mod pause;

pub fn plugin(app: &mut App) {
    //decided to always show game ui in playing gamestate
    app.add_plugins((pause::plugin));
    app.add_systems(OnEnter(Screen::Gameplay), spawn_game_ui);
}

#[derive(Component)]
struct StopwatchTime;

#[derive(Component)]
pub struct GameUi;

fn spawn_game_ui(mut commands: Commands) {
    // this does nothing essentially
    commands
        .ui_root()
        .insert((GameUi, Name::new("Game UI"), StateScoped(Screen::Gameplay)))
        .with_children(|parent| {
            let font = TextFont {
                font_size: 20.,
                ..default()
            };
            parent.spawn((
                Node {
                    flex_grow: 1.,
                    align_items: AlignItems::End,
                    ..default()
                },
                children![(
                    Node {
                        width: Val::Px(50.),
                        height: Val::Px(80.),
                        border: UiRect::all(Val::Px(10.)),
                        margin: UiRect::all(Val::Px(20.)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderColor(BLACK.into()),
                    BackgroundColor(WHITE.into()),
                    children![(
                        StopwatchTime,
                        Text::new("N/A"),
                        font,
                        TextColor(BLACK.into())
                    )]
                )],
            ));
        });
}
