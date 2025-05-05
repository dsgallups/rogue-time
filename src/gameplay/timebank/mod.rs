use avian3d::prelude::*;
use bevy::prelude::*;

use super::player::Player;

pub fn plugin(app: &mut App) {
    app.register_type::<TimeBank>()
        .register_type::<TimeBankInstance>();

    app.add_observer(insert_timebank);
}

/// This is going to be something that gives time to the user
///
/// This is inserted in blender
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TimeBank {
    pub milliseconds: u32,
}
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TimeBankInstance {
    pub milliseconds: u32,
}

fn insert_timebank(trigger: Trigger<OnAdd, TimeBank>, mut commands: Commands) {
    //can't insert sensor in blender.
    commands
        .entity(trigger.target())
        .insert((Sensor, CollisionEventsEnabled))
        .observe(collect_timebank);
}

fn collect_timebank(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    transform: Query<&Transform>,
    player: Query<&Player>,
) {
    let loc = transform.get(trigger.target()).unwrap();
    //only if the trigger was the human
    error!("Collision on timebank detected!, transform: {loc:?}");
    let event = trigger.event();
    //dont use event.body,
    if player.get(event.collider).is_err() {
        error!("Not player collider");
        return;
    }

    commands.entity(trigger.target()).despawn();
}
