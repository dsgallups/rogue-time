mod widgets;
pub use widgets::*;
pub mod interaction;

use bevy::prelude::*;
pub fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
}
