mod animation;

use std::time::Duration;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{gameplay::player::rewind::CanRewind, level::Level};

use super::{
    blender::{BlenderObject, replace_blender_object},
    interact::Interact,
    lives::LostLife,
    player::Player,
    stopwatch::Stopwatch,
};

pub fn plugin(app: &mut App) {
    app.register_type::<TimeBank>()
        .register_type::<BlenderTimebank>();

    app.add_plugins(animation::plugin);

    app.add_systems(
        PreUpdate,
        (
            replace_blender_object::<BlenderTimebank>,
            trigger_new_ephemeral_timebanks,
        ),
    );
    app.add_observer(on_add_timebank)
        .add_observer(reset_on_life_lost);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BlenderTimebank {
    pub milliseconds: u64,
    pub level: Level,
}

impl BlenderObject for BlenderTimebank {
    type BevyComponent = TimeBank;
    fn level(&self) -> Level {
        self.level
    }

    fn to_component(&self) -> Self::BevyComponent {
        TimeBank {
            milliseconds: self.milliseconds,
        }
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
        .observe(collide_with_timebank);
}
#[derive(Component)]
struct Used;

fn collide_with_timebank(
    trigger: Trigger<OnCollisionStart>,
    mut timebanks: Query<(&TimeBank, &mut Visibility), Without<Used>>,
    mut commands: Commands,
    mut player: Query<Entity, With<Player>>,
) {
    let Ok((timebank, mut visibility)) = timebanks.get_mut(trigger.target()) else {
        //used, ignore
        return;
    };

    //only if the trigger was the human
    let event = trigger.event();
    //dont use event.body,
    let Ok(player) = player.get_mut(event.collider) else {
        return;
    };
    *visibility = Visibility::Hidden;
    commands.entity(trigger.target()).insert(Used);

    commands
        .spawn(EphemeralTimebank {
            milliseconds: timebank.milliseconds,
        })
        .observe(interact_with_timebank);

    // use insert if new if we allow multiple rewinds
    commands.entity(player).insert(CanRewind);
}

// only happens for a real timebank is collected, not when replayed
fn trigger_new_ephemeral_timebanks(
    eph: Query<Entity, Added<EphemeralTimebank>>,
    mut commands: Commands,
) {
    for eph in eph {
        commands.entity(eph).trigger(Interact::default());
    }
}

/// will exist after timebank is gone so it can be retriggered
#[derive(Component)]
struct EphemeralTimebank {
    milliseconds: u64,
}

fn interact_with_timebank(
    trigger: Trigger<Interact>,
    timebanks: Query<&EphemeralTimebank>,
    mut commands: Commands,
    mut stopwatch: ResMut<Stopwatch>,
    player: Query<Entity, With<Player>>,
) {
    let timebank = timebanks.get(trigger.target()).unwrap();

    info!(
        "adding {} milliseconds to stopwatch!",
        timebank.milliseconds
    );
    stopwatch.add_time(Duration::from_millis(timebank.milliseconds));

    if let Ok(player) = player.single() {
        commands.entity(player).insert(CanRewind);
    }
}

fn reset_on_life_lost(
    _trigger: Trigger<LostLife>,
    mut commands: Commands,
    mut timebanks: Query<(Entity, &mut Visibility), With<TimeBank>>,
    ephemeral_timebanks: Query<Entity, With<EphemeralTimebank>>,
) {
    for (timebank, mut visibility) in &mut timebanks {
        *visibility = Visibility::Inherited;
        commands.entity(timebank).remove::<Used>();
    }
    for eph in ephemeral_timebanks {
        commands.entity(eph).despawn();
    }
    //todo
}
