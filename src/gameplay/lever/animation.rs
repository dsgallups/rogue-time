use bevy::{prelude::*, scene::SceneInstanceReady};

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
        .observe(play_when_ready);
}

#[derive(Component)]
struct LeverAnimation {
    graph_handle: Handle<AnimationGraph>,
    index: AnimationNodeIndex,
}

// we will have a trigger which will then trigger this on the StopWatch component being inserted...
// or maybe this happens automagically with the animation plugin via link_animation_player
//
// note the observer is not app wide
fn play_when_ready(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    animations_to_play: Query<&LeverAnimation>,
    children: Query<&Children>,
    mut players: Query<&mut AnimationPlayer>,
) {
    // The entity we spawned in `setup_mesh_and_animation` is the trigger's target.
    // Start by finding the AnimationToPlay component we added to that entity.
    if let Ok(animation_to_play) = animations_to_play.get(trigger.target()) {
        // The SceneRoot component will have spawned the scene as a hierarchy
        // of entities parented to our entity. Since the asset contained a skinned
        // mesh and animations, it will also have spawned an animation player
        // component. Search our entity's descendants to find the animation player.
        for child in children.iter_descendants(trigger.target()) {
            if let Ok(mut player) = players.get_mut(child) {
                // Tell the animation player to start the animation and keep
                // repeating it.
                //
                // If you want to try stopping and switching animations, see the
                // `animated_mesh_control.rs` example.
                player.play(animation_to_play.index).repeat();

                // Add the animation graph. This only needs to be done once to
                // connect the animation player to the mesh.
                commands
                    .entity(child)
                    .insert(AnimationGraphHandle(animation_to_play.graph_handle.clone()));
            }
        }
    }
}
