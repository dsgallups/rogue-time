use bevy::prelude::*;

mod camera;
mod input;

pub fn plugin(app: &mut App) {
    app.register_type::<Player>();

    app.add_plugins((camera::plugin, input::plugin));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player;
