use std::{f32::consts::PI, ffi::OsStr};

use bevy::{
    asset::AssetMetaCheck, prelude::*, render::view::RenderLayers, scene::SceneInstanceReady,
    window::WindowMode,
};

const UI_RENDER_LAYER: usize = 2;

/// High level groups of systems in the "Update" schedule.
///
/// Following the justifications of foxtrot, thought it would be nice to have now rather than later
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Reflect)]
enum AppSet {
    /// Tick timers
    TickTimers,
    /// Record player input
    RecordInput,
    /// do everything else
    Update,
}
fn main() {
    let mut app = App::new();

    app.register_type::<AppSet>();

    app.configure_sets(
        Update,
        (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
    );

    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Window {
                    title: "Rogue Time".to_string(),
                    fit_canvas_to_parent: true,
                    // might need to adjust this for WASM
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    ..default()
                }
                .into(),
                ..default()
            }),
    );

    //spawn ui camera. should always exist
    app.add_systems(Startup, spawn_ui_camera);

    // Bevy should rotate gltf coordinates to properly work in the system
    app.add_observer(fix_gltf_coordinates);
}

fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("UI Camera"),
        Camera2d,
        // Render all UI to this camera.
        IsDefaultUiCamera,
        // render UI to this layer to be captured by the UI camera
        RenderLayers::layer(UI_RENDER_LAYER),
        Camera {
            // Bump the order to render on top of the view model.
            order: 2,
            ..default()
        },
    ));
}

// bevy uses -Z as forward, but doesn't respect that GLTF uses +Z forward.
fn fix_gltf_coordinates(
    trigger: Trigger<SceneInstanceReady>,
    q_scene_root: Query<(&SceneRoot, &Children)>,
    mut q_transform: Query<&mut Transform>,
) {
    let scene_entity = trigger.target();
    let Ok((scene_root, children)) = q_scene_root.get(scene_entity) else {
        return;
    };

    let Some(asset_path) = scene_root.0.path() else {
        return;
    };

    let Some(extension) = asset_path.path().extension().and_then(OsStr::to_str) else {
        return;
    };

    const GLTF_EXTENSIONS: [&str; 2] = ["glb", "gltf"];
    if !GLTF_EXTENSIONS.contains(&extension) {
        return;
    }

    let mut iter = q_transform.iter_many_mut(children);
    while let Some(mut transform) = iter.fetch_next() {
        transform.rotate_y(PI);
    }
}
