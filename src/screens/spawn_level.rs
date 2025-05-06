use bevy::{prelude::*, scene::SceneInstanceReady};

use crate::{asset_tracking::LoadResource, gameplay::player::Player, theme::widgets};

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
        .add_systems(OnExit(Screen::Gameplay), unspawn_player);

    app.load_resource::<LevelAssets>()
        .init_resource::<LevelLoaded>()
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
        .spawn(SceneRoot(scene_assets.level.clone()))
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

// fn advance_to_gameplay_screen(
//     player_camera: Query<&PlayerCamera>,
//     mut next_screen: ResMut<NextState<Screen>>,
// ) {
//     if !player_camera.is_empty() {
//         next_screen.set(Screen::Gameplay);
//     }
// }

/// A [`Resource`] that contains all the assets needed to spawn the level.
/// We use this to preload assets before the level is spawned.
#[derive(Resource, Asset, Clone, TypePath)]
pub(crate) struct LevelAssets {
    #[dependency]
    pub(crate) level: Handle<Scene>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            level: assets.load(GltfAssetLabel::Scene(0).from_asset("scenes/sandbox.glb")),
        }
    }
}
