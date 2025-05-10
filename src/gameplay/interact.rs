use bevy::prelude::*;

// will replay all the events in order for a room on rewinding
pub fn plugin(app: &mut App) {
    //todo
}

// all interactions should use this event.
#[derive(Event)]
pub struct Interact;
