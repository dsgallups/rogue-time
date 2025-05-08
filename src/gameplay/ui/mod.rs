use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use crate::{screens::Screen, theme::widgets};

use super::{
    GameSet,
    lives::Lives,
    player::{Player, rewind::CanRewind},
    room::RoomCountdown,
    time::LevelTimer,
};

mod pause;

pub fn plugin(app: &mut App) {
    //decided to always show game ui in playing gamestate
    app.add_plugins((pause::plugin));
    app.add_systems(OnEnter(Screen::Gameplay), spawn_game_ui)
        .add_systems(
            Update,
            (
                update_time_ui,
                update_rewind_ui,
                update_lives_ui,
                update_countdown,
            )
                .in_set(GameSet::UiUpdate)
                .run_if(in_state(Screen::Gameplay)),
        );
}

#[derive(Component)]
struct StopwatchTimeUi;

#[derive(Component)]
pub struct GameUi;

#[derive(Component)]
pub struct LivesUi;

#[derive(Component)]
pub struct CountdownUi;

fn spawn_game_ui(mut commands: Commands) {
    let font = TextFont {
        font_size: 20.,
        ..default()
    };
    commands
        .spawn((
            widgets::ui_root("Game UI"),
            GameUi,
            StateScoped(Screen::Gameplay),
            children![
                (
                    Node {
                        width: Val::Px(200.),
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
                        LivesUi,
                        Text::new("N/A"),
                        font.clone(),
                        TextColor(BLACK.into())
                    )]
                ),
                (
                    Node {
                        width: Val::Px(200.),
                        height: Val::Px(200.),
                        border: UiRect::all(Val::Px(10.)),
                        margin: UiRect::all(Val::Px(20.)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    children![(
                        CountdownUi,
                        Text::new("N/A"),
                        TextFont {
                            font_size: 40.,
                            ..default()
                        },
                        TextColor(BLACK.into())
                    )]
                ),
                (
                    Node {
                        flex_grow: 1.,
                        align_self: AlignSelf::Stretch,
                        align_items: AlignItems::End,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    children![
                        (
                            Node {
                                width: Val::Px(80.),
                                height: Val::Px(50.),
                                border: UiRect::all(Val::Px(5.)),
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
                                font.clone(),
                                TextColor(BLACK.into())
                            )]
                        ),
                        (
                            RewindParent,
                            Node {
                                border: UiRect::all(Val::Px(10.)),
                                margin: UiRect::all(Val::Px(20.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BorderColor(BLACK.into()),
                            BackgroundColor(WHITE.into()),
                            children![(RewindUi, Text::default(), font, TextColor(BLACK.into()))]
                        ),
                    ],
                )
            ],
        ))
        .insert(BackgroundColor(Color::NONE));
}

fn update_time_ui(stopwatch: Res<LevelTimer>, mut text: Query<&mut Text, With<StopwatchTimeUi>>) {
    let Ok(mut text) = text.single_mut() else {
        warn!("Missing stopwatch UI!");
        return;
    };

    let time = stopwatch.0.remaining_secs();
    text.0 = format!("{time:.02}");
}

#[derive(Component)]
pub struct RewindParent;
#[derive(Component)]
pub struct RewindUi;
// we could make this two observers, but nahhhh
fn update_rewind_ui(
    rewindable: Query<Has<CanRewind>, With<Player>>,
    mut text: Query<(&mut Text, &mut TextColor), With<RewindUi>>,
    mut par: Query<(&mut BorderColor, &mut BackgroundColor), With<RewindParent>>,
    mut prev: Local<bool>,
) {
    // we could check against a Local<bool> but eh
    let Ok(can_rewind) = rewindable.single() else {
        //warn!("Stopwatch not detected!");
        return;
    };

    if can_rewind == *prev {
        return;
    }

    let Ok((mut border, mut background)) = par.single_mut() else {
        warn!("Missing rewind node!");
        return;
    };

    let Ok((mut text, mut text_color)) = text.single_mut() else {
        warn!("Missing rewind text!");
        return;
    };

    if can_rewind {
        text.0 = "Rewind".to_string();

        *border = WHITE.into();
        *background = BLACK.into();
        *text_color = WHITE.into();
    } else {
        text.0 = "".to_string();
        *border = BLACK.into();
        *background = WHITE.into();
        *text_color = BLACK.into();
    }
    *prev = can_rewind;
}

fn update_lives_ui(
    rewindable: Query<&Lives, Changed<Lives>>,
    mut text: Query<&mut Text, With<LivesUi>>,
) {
    let Ok(lives) = rewindable.single() else {
        return;
    };

    let Ok(mut text) = text.single_mut() else {
        warn!("Missing rewind text!");
        return;
    };
    text.0 = format!("{} lives", lives.count());
}

fn update_countdown(
    countdown: Option<Res<RoomCountdown>>,
    mut text: Query<&mut Text, With<CountdownUi>>,
) {
    let Ok(mut text) = text.single_mut() else {
        warn!("Missing countdown UI!");
        return;
    };
    let Some(countdown) = countdown else {
        text.0 = String::new();
        return;
    };

    text.0 = countdown.secs_left.to_string();
}
