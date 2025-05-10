use animation::LeverAnimation;
use avian3d::prelude::*;
use bevy::prelude::*;
mod animation;
use crate::level::Level;

use super::{
    blender::{BlenderObject, replace_blender_object},
    interact::Interact,
    portal::{KeyFor, PortalKey},
};

pub fn plugin(app: &mut App) {
    app.register_type::<BlenderLever>()
        .register_type::<Lever>()
        .add_observer(on_add_lever)
        .add_plugins(animation::plugin)
        .add_systems(PreUpdate, replace_blender_object::<BlenderLever>);
}

/// Marker type for lever with door id
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BlenderLever {
    level: Level,
}

impl BlenderObject for BlenderLever {
    type BevyComponent = Lever;
    fn level(&self) -> Level {
        self.level
    }

    fn to_component(&self) -> Self::BevyComponent {
        Lever
    }
}

/// Lever for trickering events
#[derive(Component, Reflect)]
#[require(Pickable)]
#[reflect(Component)]
pub struct Lever;

fn on_add_lever(trigger: Trigger<OnAdd, Lever>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert((PortalKey::default(), Collider::sphere(2.)))
        .observe(on_interact);
}

fn on_interact(
    trigger: Trigger<Interact>,
    mut commands: Commands,
    mut portal_keys: Query<(&mut PortalKey, &KeyFor)>,
    animations_to_play: Query<&LeverAnimation>,
    children: Query<&Children>,
    mut players: Query<&mut AnimationPlayer>,
) {
    let (mut portal_key, portal) = portal_keys.get_mut(trigger.target()).unwrap();
    portal_key.interacted = !portal_key.interacted;
    info!(
        "Interacted with lever!, switched to {}",
        portal_key.interacted
    );
    if let Ok(animation_to_play) = animations_to_play.get(trigger.target()) {
        for child in children.iter_descendants(trigger.target()) {
            if let Ok(mut player) = players.get_mut(child) {
                let animation = player.animation_mut(animation_to_play.index).unwrap();
                let seek = animation.seek_time();
                if animation.is_paused() {
                    animation.resume();
                }
                animation.replay();
                animation.seek_to(seek);
                // play the animation to the end
                if portal_key.interacted {
                    animation.set_speed(1.);
                } else {
                    animation.set_speed(-1.);
                }
            }
        }
    }
    commands.entity(portal.0).trigger(Interact);
}
