use avian3d::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<TimeBank>();

    app.add_observer(insert_timebank);
}

/// This is going to be something that gives time to the user
///
/// This is inserted in blender
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TimeBank {
    milliseconds: u32,
}

fn insert_timebank(trigger: Trigger<OnAdd, TimeBank>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert((RigidBody::Static, Collider::sphere(1.), Sensor))
        .observe(collect_timebank);
}

fn collect_timebank(trigger: Trigger<OnCollisionStart>, mut commands: Commands) {
    error!("Collision on timebank detected!");
    commands.entity(trigger.observer()).despawn();
}
