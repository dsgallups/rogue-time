use bevy::prelude::*;

use crate::{gameplay::player::Player, level::LevelLoaded, theme::widgets};

use super::Screen;

/// set to true when the player has spawned, and set to false when leaving gameplay
#[derive(Resource, Default)]
pub struct PlayerAlreadySpawned(bool);

pub fn plugin(app: &mut App) {
    // spawn the level in the background, the title screen is valuable time to speed up things
    // we're ready to go as soon as we leave the loading screen.
    app.add_systems(OnExit(Screen::Gameplay), (unspawn_player))
        .init_resource::<PlayerAlreadySpawned>();
    app.add_systems(OnEnter(Screen::SpawnLevel), spawn_spawn_level_screen)
        .add_systems(Update, spawn_player.run_if(in_state(Screen::SpawnLevel)));
    // app.add_systems(
    //     Update,
    //     advance_to_gameplay_screen.run_if(in_state(Screen::SpawnLevel)),
    // );
}

fn unspawn_player(mut player_spawned: ResMut<PlayerAlreadySpawned>) {
    player_spawned.0 = false;
}

// spawn the player when
// 1. the level has loaded
// 2. the user has clicked play
//
// This in turn will kick off a set of observers that will eventually create the player camera.
fn spawn_player(
    level_ready: Res<LevelLoaded>,
    mut player_spawned: ResMut<PlayerAlreadySpawned>,
    mut commands: Commands,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if player_spawned.0 {
        return;
    }
    if !level_ready.0 {
        return;
    }
    info!("Spawning player!");
    commands.spawn(Player);
    next_screen.set(Screen::Gameplay);
    player_spawned.0 = true;
}

fn spawn_spawn_level_screen(mut commands: Commands) {
    commands.spawn((
        widgets::ui_root("Loading Screen"),
        StateScoped(Screen::SpawnLevel),
        children![widgets::label("Spawning Level...")],
    ));
}
