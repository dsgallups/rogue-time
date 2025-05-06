pub mod interaction;
pub mod palette;
pub mod shader;
pub mod widgets;

use bevy::prelude::*;
pub fn plugin(app: &mut App) {
    //shader::shader_plugin
    app.add_plugins((interaction::plugin, shader::plugin));
}
