use std::time::Duration;

use bevy::prelude::*;

// will replay all the events in order for a room on rewinding
pub fn plugin(app: &mut App) {
    app.init_resource::<Interactions>()
        .add_observer(on_interaction);
    //todo
}

//fn log_interaction()

#[derive(Resource, Default)]
struct Interactions {
    inner: Vec<(Entity, Duration)>,
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

fn on_interaction(trigger: Trigger<Interact>, mut interactions: ResMut<Interactions>) {
    let target = trigger.target();
    let event = trigger.event();
    if event.ignore {
        return;
    }

    //interactions.inner.push(target)

    //bread and butter observer
    //todo
}
