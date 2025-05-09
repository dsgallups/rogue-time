//! Code to load all of our gltfs and scene stuffs

use bevy::{prelude::*, scene::SceneInstanceReady};

use crate::{asset_tracking::LoadResource, screens::Screen};

pub fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>()
        .init_resource::<LevelLoaded>();
    app.add_systems(OnExit(Screen::Loading), spawn_level)
        .add_systems(OnExit(Screen::Gameplay), respawn_level);
}

#[derive(Resource, Default)]
pub struct LevelLoaded(pub bool);

/// A [`Resource`] that contains all the assets needed to spawn the level.
/// We use this to preload assets before the level is spawned.
#[derive(Resource, Asset, Clone, TypePath)]
pub(crate) struct LevelAssets {
    #[dependency]
    pub(crate) level0: Handle<Scene>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            level0: assets.load(GltfAssetLabel::Scene(0).from_asset("levels/level0.glb")),
        }
    }
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
