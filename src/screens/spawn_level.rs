use bevy::{prelude::*, scene::SceneInstanceReady};

use crate::{
    gameplay::player::{Player, camera::PlayerCamera},
    theme::widgets,
};

use super::Screen;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::SpawnLevel),
        (spawn_level, spawn_spawn_level_screen),
    );
    app.add_systems(
        Update,
        advance_to_gameplay_screen.run_if(in_state(Screen::SpawnLevel)),
    );
}

fn spawn_level(mut commands: Commands, scene_assets: Res<LevelAssets>) {
    commands
        .spawn(SceneRoot(scene_assets.level.clone()))
        .observe(spawn_player);

    commands.spawn((
        PointLight {
            intensity: 5000.,
            ..default()
        },
        Transform::default(),
    ));
}
// spawn the player when the level has loaded.
//
// This in turn will kick off a set of observers that will eventually create the player camera.
fn spawn_player(_trigger: Trigger<SceneInstanceReady>, mut commands: Commands) {
    commands.spawn(Player);
}

fn spawn_spawn_level_screen(mut commands: Commands) {
    commands.spawn((
        widgets::ui_root("Loading Screen"),
        StateScoped(Screen::SpawnLevel),
        children![widgets::label("Spawning Level...")],
    ));
}

fn advance_to_gameplay_screen(
    player_camera: Query<&PlayerCamera>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if !player_camera.is_empty() {
        next_screen.set(Screen::Gameplay);
    }
}

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
