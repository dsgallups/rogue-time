use bevy::prelude::*;

pub mod lever;
// mod pressure_plate;
// mod elevator;

pub fn plugin(app: &mut App) {
    app.add_plugins(lever::plugin);
}
