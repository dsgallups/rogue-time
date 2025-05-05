use bevy::{
    color::palettes::css::{BLACK, RED, WHITE},
    prelude::*,
};

use crate::{AppSet, screens::Screen, theme::Containers};

use super::{GameSet, stopwatch::StopwatchTimer};

mod pause;

pub fn plugin(app: &mut App) {
    //decided to always show game ui in playing gamestate
    app.add_plugins((pause::plugin));
    app.add_systems(OnEnter(Screen::Gameplay), spawn_game_ui)
        .add_systems(
            Update,
            update_time_ui
                .in_set(GameSet::Update)
                .run_if(in_state(Screen::Gameplay)),
        );
}

#[derive(Component)]
struct StopwatchTimeUi;

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
                        width: Val::Px(80.),
                        height: Val::Px(50.),
                        border: UiRect::all(Val::Px(10.)),
                        margin: UiRect::all(Val::Px(20.)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderColor(BLACK.into()),
                    BackgroundColor(WHITE.into()),
                    children![(
                        StopwatchTimeUi,
                        Text::new("N/A"),
                        font,
                        TextColor(BLACK.into())
                    )]
                )],
            ));
        });
}

fn update_time_ui(
    stopwatch: Query<&StopwatchTimer>,
    mut text: Query<&mut Text, With<StopwatchTimeUi>>,
) {
    let Ok(stopwatch) = stopwatch.single() else {
        warn!("Stopwatch not detected!");
        return;
    };
    let Ok(mut text) = text.single_mut() else {
        warn!("Missing stopwatch UI!");
        return;
    };

    let time = stopwatch.0.remaining_secs();
    text.0 = format!("{time:.02}");
}
