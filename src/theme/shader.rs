use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

// const DEFAULT_COLOR1: LinearRgba = LinearRgba::GREEN;

pub fn shader_plugin(app: &mut App) {
    app
        // .register_type::<OneBitColor>()
        .add_plugins((
            // MaterialPlugin::<DitheredMaterial>::default(),
            bevy_dither_post_process::DitherPostProcessPlugin,
        ))
        .add_systems(Update, insert_post_process)
    // .add_systems(Update, set_color)
    ;
}

fn insert_post_process(
    query: Query<(Entity), Added<Camera3d>>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    for camera in query {
        commands.entity(camera).insert(
            bevy_dither_post_process::components::DitherPostProcessSettings::new(3, &asset_server),
        );
    }
}

// Dither texture shader
// #[derive(Component, Reflect)]
// #[reflect(Component)]
// pub struct OneBitColor;

// #[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
// struct DitheredMaterial {
//     #[uniform(0)]
//     color: LinearRgba,
// }
// fn set_color(
//     query: Query<(Entity), Added<OneBitColor>>,
//     mut commands: Commands,
//     mut materials: ResMut<Assets<DitheredMaterial>>,
// ) {
//     for entity in query {
//         commands
//             .entity(entity)
//             .insert(MeshMaterial3d(materials.add(DitheredMaterial {
//                 color: DEFAULT_COLOR1,
//             })));
//         info!("found dithered entity");
//     }
// }

// impl Material for DitheredMaterial {
//     fn fragment_shader() -> ShaderRef {
//         "shaders/dithering.wgsl".into()
//     }
// }
