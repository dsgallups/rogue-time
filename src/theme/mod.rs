pub mod interaction;
pub mod palette;
// #[cfg(all(not(feature = "dev"), feature = "native"))]
pub mod shader;
pub mod widgets;

use bevy::prelude::*;
pub fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
    // #[cfg(all(not(feature = "dev"), feature = "native"))]
    app.add_plugins(shader::plugin);
}
