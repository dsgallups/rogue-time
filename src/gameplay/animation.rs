//! Universal animation props
//!
//! Animation system boilerplate.
use std::iter;

use bevy::{prelude::*, scene::SceneInstanceReady};

use crate::screens::Screen;

pub fn plugin(app: &mut App) {
    app.register_type::<AnimationBusy>()
        .register_type::<AnimationPlayers>()
        .register_type::<AnimationPlayerOf>();

    app.add_systems(
        PreUpdate,
        tick_animation_busy.run_if(in_state(Screen::Gameplay)),
    );
    app.register_type::<AnimationPlayerOf>();
    app.register_type::<AnimationPlayers>();
    app.add_observer(link_animation_player);
}

#[derive(Component, Reflect)]
pub struct AnimationBusy(pub Timer);

pub fn tick_animation_busy(
    mut commands: Commands,
    time: Res<Time>,
    mut animation_busy: Query<(Entity, &mut AnimationBusy)>,
) {
    for (entity, mut anim_busy) in &mut animation_busy {
        anim_busy.0.tick(time.delta());
        if anim_busy.0.finished() {
            info!("Animation is no longer busy");
            commands.entity(entity).remove::<AnimationBusy>();
        }
    }
}

/// Entities with this component will receive an [`AnimationPlayers`] relationship so that they can easily find the animation player of their model.
#[derive(Component, Reflect)]
pub(crate) struct AnimationPlayerAncestor;

/// Simple link to the animation player of a model that is buried deep in the hierarchy.
#[derive(Component, Reflect, Clone, Deref)]
#[reflect(Component)]
#[relationship_target(relationship = AnimationPlayerOf)]
pub(crate) struct AnimationPlayers(Vec<Entity>);

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
#[relationship(relationship_target = AnimationPlayers)]
pub(crate) struct AnimationPlayerOf(pub(crate) Entity);

/// Bevy likes to hide the [`AnimationPlayer`] component deep in the hierarchy of a model.
/// This system ensures that we can find the animation player easily by inserting an [`AnimationPlayers`] relationship
/// into the same entity that contains the [`AnimationPlayerAncestor`] component.
fn link_animation_player(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    q_parent: Query<&ChildOf>,
    q_children: Query<&Children>,
    q_animation_player: Query<Entity, With<AnimationPlayer>>,
    q_ancestor: Query<Entity, With<AnimationPlayerAncestor>>,
) {
    error!("linking anim players");
    let scene_root = trigger.target();
    let animation_player = q_children
        .iter_descendants(scene_root)
        .find(|child| q_animation_player.get(*child).is_ok());
    let Some(animation_player) = animation_player else {
        error!("couldnt link anim players 1");
        return;
    };

    let animation_ancestor = iter::once(animation_player)
        .chain(q_parent.iter_ancestors(animation_player))
        .find(|entity| q_ancestor.get(*entity).is_ok());
    let Some(animation_ancestor) = animation_ancestor else {
        error!("couldnt link anim players 2");
        return;
    };

    commands
        .entity(animation_player)
        .insert(AnimationPlayerOf(animation_ancestor));
}
