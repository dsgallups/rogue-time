use std::time::Duration;

use bevy::prelude::*;

use super::{lives::LostLife, player::rewind::EndRewind, room::RoomStarted, stopwatch::Stopwatch};

// will replay all the events in order for a room on rewinding
pub fn plugin(app: &mut App) {
    app.init_resource::<Interactions>()
        .add_observer(on_interaction)
        .add_observer(on_lost_life)
        .add_observer(on_room_start)
        .add_observer(on_rewind)
        .add_systems(Update, replay);
    //todo
}

//fn log_interaction()

#[derive(Resource, Default)]
struct Interactions {
    // ones to replay
    past: Vec<(Entity, Duration)>,
    //ones to replay on rewind
    present: Vec<(Entity, Duration)>,
    //cursor into past
    cursor: usize,
}

impl Interactions {
    fn reset(&mut self) {
        self.past.clear();
        self.present.clear();
        self.cursor = 0;
    }
}

#[derive(Component)]
struct IgnoreInteraction;

// all interactions should use this event.
#[derive(Event, Default)]
pub struct Interact {
    ignore: bool,
}

impl Interact {
    // useful for the portal and for us
    pub fn dont_record() -> Self {
        Self { ignore: true }
    }
}

fn on_interaction(
    trigger: Trigger<Interact>,
    mut interactions: ResMut<Interactions>,
    stopwatch: Res<Stopwatch>,
) {
    let target = trigger.target();
    let event = trigger.event();
    if event.ignore {
        return;
    }

    interactions.present.push((target, stopwatch.elapsed()));

    //bread and butter observer
    //todo
}
fn on_rewind(_trigger: Trigger<EndRewind>, mut interactions: ResMut<Interactions>) {
    let present_actions = interactions.present.clone();
    interactions.present.clear();
    interactions.past.extend(present_actions);
    interactions.past.sort_by_key(|a| a.1);
    interactions.cursor = 0;
}

fn on_lost_life(_trigger: Trigger<LostLife>, mut interactions: ResMut<Interactions>) {
    interactions.reset();
}

fn on_room_start(_trigger: Trigger<RoomStarted>, mut interactions: ResMut<Interactions>) {
    interactions.reset();
}

fn replay(
    mut interactions: ResMut<Interactions>,
    mut commands: Commands,
    stopwatch: Res<Stopwatch>,
) {
    let Some((entity, duration)) = interactions.past.get(interactions.cursor) else {
        // can happen for many reasons
        return;
    };
    if *duration <= stopwatch.elapsed() {
        warn!("Replaying event!");
        commands.entity(*entity).trigger(Interact::dont_record());
        interactions.cursor += 1;
    }
}
