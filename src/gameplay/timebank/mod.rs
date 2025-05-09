mod animation;

use std::time::Duration;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::gameplay::player::rewind::CanRewind;

use super::{player::Player, time::LevelTimer};

pub fn plugin(app: &mut App) {
    app.register_type::<TimeBank>()
        .register_type::<BlenderTimebank>();

    app.add_plugins(animation::plugin);

    app.add_systems(PreUpdate, on_add_blender_timebank);
    app.add_observer(on_add_timebank);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BlenderTimebank {
    pub milliseconds: u64,
}

fn on_add_blender_timebank(
    mut commands: Commands,
    blender_timebanks: Query<(Entity, &Transform, &BlenderTimebank)>,
) {
    for (entity, transform, timebank) in blender_timebanks {
        // we are going to take this thing,
        // remove it from the scene entirely,
        // and then construct it ourselves.
        //let BlenderTimebank { milliseconds } = blender_timebanks.get(trigger.target()).unwrap();
        info!("added blender timebank");

        commands.entity(entity).despawn();
        commands.spawn((
            TimeBank {
                milliseconds: timebank.milliseconds,
            },
            *transform,
        ));
    }
}

/// This is going to be something that gives time to the user
///
/// This is inserted in blender
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TimeBank {
    pub milliseconds: u64,
}

fn on_add_timebank(trigger: Trigger<OnAdd, TimeBank>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert((
            Sensor,
            RigidBody::Static,
            CollisionEventsEnabled,
            ColliderConstructor::Cylinder {
                radius: 1.,
                height: 2.,
            },
        ))
        .observe(collect_timebank);
}

fn collect_timebank(
    trigger: Trigger<OnCollisionStart>,
    timebanks: Query<&TimeBank>,
    mut commands: Commands,
    mut stopwatch: ResMut<LevelTimer>,
    mut player: Query<Entity, With<Player>>,
) {
    let timebank = timebanks.get(trigger.target()).unwrap();
    //only if the trigger was the human
    let event = trigger.event();
    //dont use event.body,
    let Ok(player) = player.get_mut(event.collider) else {
        return;
    };

    info!(
        "adding {} milliseconds to stopwatch!",
        timebank.milliseconds
    );
    stopwatch.add_time(Duration::from_millis(timebank.milliseconds));

    commands.entity(trigger.target()).despawn();

    // use insert if new if we allow multiple rewinds
    commands.entity(player).insert(CanRewind);
}
