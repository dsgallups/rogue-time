//! Code to load all of our gltfs and scene stuffs

use std::collections::HashMap;

use assets::LevelAssets;
use bevy::{prelude::*, scene::SceneInstanceReady};

use crate::screens::Screen;

mod assets;

pub fn plugin(app: &mut App) {
    app.add_plugins(assets::plugin);

    app.init_resource::<LevelsLoaded>().register_type::<Level>();

    app.add_systems(OnExit(Screen::Loading), spawn_level)
        .add_systems(OnExit(Screen::Gameplay), respawn_level);
}

#[derive(Resource)]
pub struct LevelSpawnPoints(HashMap<Level, Vec3>);

impl LevelSpawnPoints {
    // panics if not found, but like we totally control this.
    pub fn get_spawn_point(&self, level: Level) -> Vec3 {
        self.0.get(&level).copied().unwrap()
    }
}

impl Default for LevelSpawnPoints {
    fn default() -> Self {
        let mut map: HashMap<Level, Vec3> = HashMap::with_capacity(5);

        for i in 0..5 {
            map.insert(Level(0), Vec3::new(0., 3., (i as f32) * 120.));
        }

        Self(map)
    }
}

/// Associated something with a particular level.
///
/// One of the levels will be designated as a win
#[derive(Component, Reflect, Hash, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub struct Level(pub u8);

#[derive(Resource, Default)]
pub struct LevelsLoaded(pub bool);

fn spawn_level(mut commands: Commands, scene_assets: Res<LevelAssets>) {
    commands
        .spawn(SceneRoot(scene_assets.level0.clone()))
        .observe(announce_ready);

    commands.spawn((
        PointLight {
            intensity: 5000.,
            ..default()
        },
        Transform::default(),
    ));
}

fn announce_ready(_trigger: Trigger<SceneInstanceReady>, mut res: ResMut<LevelsLoaded>) {
    info!("Scene is ready!");
    res.0 = true;
}

fn respawn_level(
    mut commands: Commands,
    scenes: Query<(Entity, Option<&ChildOf>), With<SceneRoot>>,
    scene_assets: Res<LevelAssets>,
) {
    for (scene, child_of) in scenes {
        if let Some(child_of) = child_of {
            commands.entity(child_of.parent()).despawn();
        } else {
            commands.entity(scene).despawn();
        }
    }

    commands
        .spawn(SceneRoot(scene_assets.level0.clone()))
        .observe(announce_ready);

    commands.spawn((
        PointLight {
            intensity: 5000.,
            ..default()
        },
        Transform::default(),
    ));
}
