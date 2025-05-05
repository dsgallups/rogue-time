use bevy::prelude::*;

use crate::{asset_tracking::LoadResource, gameplay::animation::AnimationPlayers};

pub fn plugin(app: &mut App) {
    app.load_resource::<StopwatchAnimationAssets>();
    app.register_type::<StopwatchAnimationAssets>();
    app.register_type::<StopwatchAnimations>();
}
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct StopwatchAnimationAssets {
    #[dependency]
    pub click_animation: Handle<AnimationClip>,
    #[dependency]
    pub ticking_animation: Handle<AnimationClip>,
}

impl FromWorld for StopwatchAnimationAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            click_animation: assets
                .load(GltfAssetLabel::Animation(0).from_asset("scenes/Stopwatch.glb")),
            ticking_animation: assets
                .load(GltfAssetLabel::Animation(1).from_asset("scenes/Stopwatch.glb")),
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
struct StopwatchAnimations {
    click: AnimationNodeIndex,
    ticking: AnimationNodeIndex,
}
// we will have a trigger will then trigger this on the StopWatch component being inserted...
// or maybe this happens automagically with the animation plugin via link_animation_player
pub(super) fn setup_stopwatch_animation(
    trigger: Trigger<OnAdd, AnimationPlayers>,
    q_anim_players: Query<&AnimationPlayers>,
    mut commands: Commands,
) {
    println!("animation player")
}
