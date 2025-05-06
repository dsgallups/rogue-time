use bevy::prelude::*;

pub(super) fn plugin(_app: &mut App) {}

#[derive(Component)]
pub struct CanRewind;

#[derive(Event)]
pub struct Rewind;
