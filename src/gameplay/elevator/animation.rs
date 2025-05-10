use bevy::{animation::RepeatAnimation, prelude::*, scene::SceneInstanceReady};

use crate::asset_tracking::LoadResource;

use super::Elevator;

pub fn plugin(app: &mut App) {
    app.load_resource::<ElevatorAnimationAssets>();
    app.register_type::<ElevatorAnimationAssets>();

    app.add_observer(setup_animation);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct ElevatorAnimationAssets {
    #[dependency]
    pub model: Handle<Scene>,
    #[dependency]
    pub lift: Handle<AnimationClip>,
}

impl FromWorld for ElevatorAnimationAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            model: assets.load(GltfAssetLabel::Scene(0).from_asset("scenes/Lift.glb")),
            lift: assets.load(GltfAssetLabel::Animation(0).from_asset("scenes/Lift.glb")),
        }
    }
}

fn setup_animation(
    trigger: Trigger<OnAdd, Elevator>,
    mut commands: Commands,
    assets: Res<ElevatorAnimationAssets>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    info!("Triggered animation setup for time!");
    let (graph, index) = AnimationGraph::from_clip(assets.lift.clone());

    let graph_handle = graphs.add(graph);

    let animation_to_play = ElevatorAnimation {
        graph_handle,
        index,
    };

    commands
        .entity(trigger.target())
        .insert((animation_to_play, SceneRoot(assets.model.clone())))
        .observe(ready_animation);
}

fn ready_animation(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    animations_to_play: Query<&ElevatorAnimation>,
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
                    .pause()
                    .set_repeat(RepeatAnimation::Never);

                commands
                    .entity(child)
                    .insert(AnimationGraphHandle(animation_to_play.graph_handle.clone()));
            }
        }
    }
}

#[derive(Component)]
pub struct ElevatorAnimation {
    graph_handle: Handle<AnimationGraph>,
    pub index: AnimationNodeIndex,
}
