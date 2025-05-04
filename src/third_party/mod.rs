use bevy::prelude::*;

pub mod avian3d;
pub mod bevy_tnua;
pub mod enhanced_input;

/// A set plugin to handle our third party plugins
pub fn plugin(app: &mut App) {
    app.add_plugins((avian3d::plugin, bevy_tnua::plugin, enhanced_input::plugin));
}
