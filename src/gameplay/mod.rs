use bevy::{
    input::common_conditions::input_just_pressed,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

mod animation;
mod door;
mod interactables;
mod lives;
pub mod player;
mod portal;
mod respawn;
mod room;
pub mod stopwatch;
pub mod time;
mod timebank;
mod timelines;
mod ui;

#[cfg(feature = "dev")]
mod dev;

use crate::{AppSet, screens::Screen};

// pausing and playing...we could get rid of the pause state possibly
#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default, Reflect)]
#[source(Screen = Screen::Gameplay)]
pub enum GameState {
    #[default]
    Playing,
    Rewinding,
    // this Debug state will be useful for later
    #[cfg(feature = "dev")]
    Debug,
    Paused,
}

/// High level groups of systems in the "Update" schedule that ONLY run when Screen::Game is enabled.
///
/// Following the justifications of foxtrot, thought it would be nice to have now rather than later
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Reflect)]
enum GameSet {
    /// Tick timers
    TickTimers,
    /// Record player input
    RecordInput,
    /// do everything else
    UiUpdate,
}

pub fn plugin(app: &mut App) {
    app.register_type::<GameState>().register_type::<GameSet>();

    app.add_sub_state::<GameState>();
    app.enable_state_scoped_entities::<GameState>();

    app.configure_sets(
        Update,
        (GameSet::TickTimers, GameSet::RecordInput, GameSet::UiUpdate)
            .chain()
            .run_if(in_state(Screen::Gameplay)),
    );

    app.add_plugins((
        ui::plugin,
        stopwatch::plugin,
        player::plugin,
        room::plugin,
        animation::plugin,
        timebank::plugin,
        portal::plugin,
        respawn::plugin,
        lives::plugin,
        time::plugin,
        timelines::plugin,
        interactables::plugin,
        door::plugin,
    ));
    #[cfg(feature = "dev")]
    app.add_plugins(dev::plugin);

    // systems to grab the cursor in the play state
    app.add_systems(OnEnter(GameState::Playing), (grab_cursor));
    app.add_systems(OnExit(GameState::Playing), relinquish_cursor);
    app.add_systems(
        Update,
        pause_game
            .run_if(in_state(GameState::Playing).and(input_just_pressed(KeyCode::Escape)))
            .in_set(AppSet::Update),
    );
}

fn grab_cursor(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut().unwrap();

    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;

    // also hide the cursor
    primary_window.cursor_options.visible = false;
}

fn relinquish_cursor(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut().unwrap();

    primary_window.cursor_options.grab_mode = CursorGrabMode::None;
    primary_window.cursor_options.visible = true;
}

fn pause_game(mut next_screen: ResMut<NextState<GameState>>) {
    next_screen.set(GameState::Paused);
}
