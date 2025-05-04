use bevy::prelude::*;
use bevy_skein::SkeinPlugin;

/// This plugin will init a protocol to communicate with the blender extension.
///
/// This enables us to add components within the actual blend file, and then reflect that
/// in the imported glb/gltf files
pub fn plugin(app: &mut App) {
    app.add_plugins(SkeinPlugin::default());
    // this is a TODO
    //#[cfg(feature = "dev")]
    //app.add_plugins(SkeinPlugin { handle_brp: true });
}
