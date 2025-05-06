mod widgets;
pub use widgets::*;
pub mod interaction;
pub mod shader;

use bevy::prelude::*;
pub fn plugin(app: &mut App) {
    app.add_plugins((interaction::plugin, shader::shader_plugin));
}
