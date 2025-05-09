use bevy::{animation::RepeatAnimation, prelude::*, scene::SceneInstanceReady};

use crate::asset_tracking::LoadResource;

use super::Lever;

pub fn plugin(app: &mut App) {
    app.load_resource::<LeverAnimationAssets>();
    app.register_type::<LeverAnimationAssets>();

    app.add_observer(setup_animation);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct LeverAnimationAssets {
    #[dependency]
    pub model: Handle<Scene>,
    #[dependency]
    pub spin: Handle<AnimationClip>,
}

impl FromWorld for LeverAnimationAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            model: assets.load(GltfAssetLabel::Scene(0).from_asset("scenes/Lever.glb")),
            spin: assets.load(GltfAssetLabel::Animation(0).from_asset("scenes/Lever.glb")),
        }
    }
}

fn setup_animation(
    trigger: Trigger<OnAdd, Lever>,
    mut commands: Commands,
    assets: Res<LeverAnimationAssets>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    info!("Triggered animation setup for Lever!");
    let (graph, index) = AnimationGraph::from_clip(assets.spin.clone());

    let graph_handle = graphs.add(graph);

    let animation_to_play = LeverAnimation {
        graph_handle,
        index,
    };

    commands
        .entity(trigger.target())
        .insert((animation_to_play, SceneRoot(assets.model.clone())))
        .observe(ready_animation);
}

#[derive(Component)]
pub struct LeverAnimation {
    pub graph_handle: Handle<AnimationGraph>,
    pub index: AnimationNodeIndex,
}

// we will have a trigger which will then trigger this on the StopWatch component being inserted...
// or maybe this happens automagically with the animation plugin via link_animation_player
//
// note the observer is not app wide
fn ready_animation(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    animations_to_play: Query<&LeverAnimation>,
    children: Query<&Children>,
    mut players: Query<&mut AnimationPlayer>,
) {
    if let Ok(animation_to_play) = animations_to_play.get(trigger.target()) {
        for child in children.iter_descendants(trigger.target()) {
            // Add the animation graph. This only needs to be done once to
            // connect the animation player to the mesh.
            //

            if let Ok(mut player) = players.get_mut(child) {
                //set the animation, but start it paused.
                player
                    .play(animation_to_play.index)
                    .set_repeat(RepeatAnimation::Never);

                commands
                    .entity(child)
                    .insert(AnimationGraphHandle(animation_to_play.graph_handle.clone()));
            }
        }
    }
}
