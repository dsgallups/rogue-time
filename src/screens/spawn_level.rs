use bevy::{prelude::*, scene::SceneInstanceReady};

use crate::{gameplay::player::Player, scene::LevelAssets, theme::widgets};

use super::Screen;

#[derive(Resource, Default)]
struct LevelLoaded(bool);

/// set to true when the player has spawned, and set to false when leaving gameplay
#[derive(Resource, Default)]
pub struct PlayerAlreadySpawned(bool);

pub fn plugin(app: &mut App) {
    // spawn the level in the background, the title screen is valuable time to speed up things
    // we're ready to go as soon as we leave the loading screen.
    app.add_systems(OnExit(Screen::Loading), spawn_level)
        .add_systems(OnExit(Screen::Gameplay), (unspawn_player, respawn_level));

    app.init_resource::<LevelLoaded>()
        .init_resource::<PlayerAlreadySpawned>();
    app.add_systems(OnEnter(Screen::SpawnLevel), spawn_spawn_level_screen)
        .add_systems(Update, spawn_player.run_if(in_state(Screen::SpawnLevel)));
    // app.add_systems(
    //     Update,
    //     advance_to_gameplay_screen.run_if(in_state(Screen::SpawnLevel)),
    // );
}

fn spawn_level(mut commands: Commands, scene_assets: Res<LevelAssets>) {
    commands
        .spawn(SceneRoot(scene_assets.level0.clone()))
        .observe(announce_ready);

    commands.spawn((
        PointLight {
            intensity: 5000.,
            ..default()
        },
        Transform::default(),
    ));
}

fn announce_ready(_trigger: Trigger<SceneInstanceReady>, mut res: ResMut<LevelLoaded>) {
    info!("Scene is ready!");
    res.0 = true;
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

fn respawn_level(
    mut commands: Commands,
    scenes: Query<(Entity, Option<&ChildOf>), With<SceneRoot>>,
    scene_assets: Res<LevelAssets>,
) {
    for (scene, child_of) in scenes {
        if let Some(child_of) = child_of {
            commands.entity(child_of.parent()).despawn();
        } else {
            commands.entity(scene).despawn();
        }
    }

    commands
        .spawn(SceneRoot(scene_assets.level0.clone()))
        .observe(announce_ready);

    commands.spawn((
        PointLight {
            intensity: 5000.,
            ..default()
        },
        Transform::default(),
    ));
}
