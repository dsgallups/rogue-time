use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<Stopwatch>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Stopwatch;
