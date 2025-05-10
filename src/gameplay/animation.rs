//! Universal animation props
//!
//! Animation system boilerplate.

use bevy::prelude::*;

use crate::screens::Screen;

pub fn plugin(app: &mut App) {
    app.register_type::<AnimationBusy>()
        .register_type::<AnimationPlayerAncestor>()
        .register_type::<AnimationPlayers>()
        .register_type::<AnimationPlayerOf>();

    app.add_systems(
        PreUpdate,
        tick_animation_busy.run_if(in_state(Screen::Gameplay)),
    );
    //app.add_observer(link_animation_player);
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
