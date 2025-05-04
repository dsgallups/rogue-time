use bevy::{
    color::palettes::css::BLACK, ecs::relationship::RelatedSpawnerCommands, prelude::*,
    render::view::RenderLayers,
};

use crate::UI_RENDER_LAYER;

/// An extension trait for spawning UI widgets.
pub(crate) trait Widgets {
    /// Spawn a simple button with text.
    fn button(&mut self, text: impl Into<String>) -> EntityCommands;

    // /// Spawn a simple header label. Bigger than [`Widgets::label`].
    // fn header(&mut self, text: impl Into<String>) -> EntityCommands;

    // /// Spawn a simple text label.
    // fn label(&mut self, text: impl Into<String>) -> EntityCommands;
}

impl<T: Spawn> Widgets for T {
    fn button(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Button"),
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(65.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            RenderLayers::layer(UI_RENDER_LAYER),
        ));
        entity.with_children(|children| {
            children.spawn((
                Name::new("Button Text"),
                Text(text.into()),
                TextFont::from_font_size(40.0),
                TextColor(BLACK.into()),
                RenderLayers::layer(UI_RENDER_LAYER),
            ));
        });

        entity
    }

    // fn header(&mut self, text: impl Into<String>) -> EntityCommands {
    //     let mut entity = self.spawn((
    //         Name::new("Header"),
    //         Node {
    //             width: Px(500.0),
    //             height: Px(65.0),
    //             justify_content: JustifyContent::Center,
    //             align_items: AlignItems::Center,
    //             ..default()
    //         },
    //         BackgroundColor(NODE_BACKGROUND),
    //         RenderLayers::layer(UI_RENDER_LAYER),
    //     ));
    //     entity.with_children(|children| {
    //         ChildBuild::spawn(
    //             children,
    //             (
    //                 Name::new("Header Text"),
    //                 Text(text.into()),
    //                 TextFont::from_font_size(40.0),
    //                 TextColor(HEADER_TEXT),
    //                 RenderLayers::layer(UI_RENDER_LAYER),
    //             ),
    //         );
    //     });
    //     entity
    // }

    // fn label(&mut self, text: impl Into<String>) -> EntityCommands {
    //     let entity = self.spawn((
    //         Name::new("Label"),
    //         Text(text.into()),
    //         TextFont::from_font_size(24.0),
    //         TextColor(LABEL_TEXT),
    //         Node {
    //             width: Px(500.0),
    //             ..default()
    //         },
    //         RenderLayers::layer(UI_RENDER_LAYER),
    //     ));
    //     entity
    // }
}

/// An internal trait for types that can spawn entities.
/// This is here so that [`Widgets`] can be implemented on all types that
/// are able to spawn entities.
/// Ideally, this trait should be [part of Bevy itself](https://github.com/bevyengine/bevy/issues/14231).
trait Spawn {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl Spawn for RelatedSpawnerCommands<'_, ChildOf> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        RelatedSpawnerCommands::spawn(self, bundle)
    }
}

/// An extension trait for spawning UI containers.
pub trait Containers {
    /// Spawns a root node that covers the full screen
    /// and centers its content horizontally and vertically.
    fn ui_root(&mut self) -> EntityCommands;
}

impl Containers for Commands<'_, '_> {
    fn ui_root(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("UI Root"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            RenderLayers::layer(UI_RENDER_LAYER),
        ))
    }
}
