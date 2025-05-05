use bevy::{asset::AssetMetaCheck, prelude::*, render::view::RenderLayers, window::WindowMode};
use bitflags::bitflags;

mod asset_tracking;
mod gameplay;
mod screens;
mod theme;
mod third_party;

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

    //other plugins
    app.add_plugins((
        third_party::plugin,
        theme::plugin,
        screens::plugin,
        asset_tracking::plugin,
    ));

    //spawn ui camera. should always exist
    app.add_systems(Startup, spawn_ui_camera);

    // Bevy should rotate gltf coordinates to properly work in the system
    //app.add_observer(fix_gltf_coordinates);

    app.run();
}

impl From<CameraOrder> for isize {
    fn from(order: CameraOrder) -> Self {
        order as isize
    }
}
fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("UI Camera"),
        Camera2d,
        // Render all UI to this camera.
        IsDefaultUiCamera,
        Camera {
            // Bump the order to render on top of the view model.
            order: CameraOrder::Ui.into(),
            ..default()
        },
    ));
}

/// This enum is converted to an `isize` to be used as a camera's order.
/// Since we have three cameras, we use three enum variants.
/// This ordering here mean UI > ViewModel > World.
enum CameraOrder {
    World,
    ViewModel,
    Ui,
}

bitflags! {
    struct RenderLayer: u32 {
        /// Used implicitly by all entities without a `RenderLayers` component.
        /// Our world model camera and all objects other than the player are on this layer.
        /// The light source belongs to both layers.
        const DEFAULT = 0b00000001;
        /// Used by the view model camera and the player's arm.
        /// The light source belongs to both layers.
        const VIEW_MODEL = 0b00000010;
        /// Since we use multiple cameras, we need to be explicit about
        /// which one is allowed to render particles.
        const PARTICLES = 0b00000100;
        /// Skip interaction with lights
        const TRANSLUCENT = 0b00001000;
    }
}

impl From<RenderLayer> for RenderLayers {
    fn from(layer: RenderLayer) -> Self {
        // Bevy's default render layer is 0, so we need to subtract 1 from our bitfalgs to get the correct value.
        RenderLayers::from_iter(layer.iter().map(|l| l.bits() as usize - 1))
    }
}
