use bevy::prelude::*;

use crate::asset_tracking::LoadResource;

pub fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();

    app.register_type::<Level0>()
        .register_type::<Level1>()
        .register_type::<Level2>();
}

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

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Level0;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Level1;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Level2;
