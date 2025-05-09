use avian3d::prelude::*;
use bevy::prelude::*;
mod animation;
use crate::level::Level;

use super::{
    blender::{BlenderObject, replace_blender_object},
    portal::PortalKey,
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
    //let level = levels.get(trigger.target()).unwrap();

    commands
        .entity(trigger.target())
        .insert((PortalKey::default(), Collider::sphere(2.)));
}
