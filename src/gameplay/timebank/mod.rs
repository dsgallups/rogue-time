use std::time::Duration;

use avian3d::prelude::*;
use bevy::prelude::*;

use super::stopwatch::StopwatchTimer;

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
    pub milliseconds: u64,
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
    timebanks: Query<&TimeBank>,
    mut commands: Commands,
    transform: Query<&Transform>,
    mut stopwatch: Query<&mut StopwatchTimer>,
) {
    let timebank = timebanks.get(trigger.target()).unwrap();
    let loc = transform.get(trigger.target()).unwrap();
    //only if the trigger was the human
    error!("Collision on timebank detected!, transform: {loc:?}");
    let event = trigger.event();
    //dont use event.body,
    let Ok(mut stopwatch) = stopwatch.get_mut(event.collider) else {
        return;
    };

    stopwatch.add_time(Duration::from_millis(timebank.milliseconds));

    commands.entity(trigger.target()).despawn();
}
