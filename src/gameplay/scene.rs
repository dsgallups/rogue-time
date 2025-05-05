use bevy::prelude::*;

use crate::asset_tracking::LoadResource;
pub fn plugin(app: &mut App) {
    app.load_resource::<SceneAssets>();
    app.add_systems(Startup, spawn_scene);
}

fn spawn_scene(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn(SceneRoot(scene_assets.level.clone()));

    commands.spawn((
        PointLight {
            intensity: 5000.,
            ..default()
        },
        Transform::default(),
    ));
}
/// A [`Resource`] that contains all the assets needed to spawn the level.
/// We use this to preload assets before the level is spawned.
#[derive(Resource, Asset, Clone, TypePath)]
pub(crate) struct SceneAssets {
    #[dependency]
    pub(crate) level: Handle<Scene>,
}

impl FromWorld for SceneAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            level: assets.load(GltfAssetLabel::Scene(0).from_asset("scenes/sandbox.glb")),
        }
    }
}
