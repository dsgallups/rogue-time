use crate::{
    gameplay::{interact::Interact, player::Player},
    level::Level,
};
use avian3d::prelude::*;
use bevy::prelude::*;

use super::{
    blender::{BlenderObject, replace_blender_object},
    elevator::{ElevatorKey, KeyFor},
};

pub fn plugin(app: &mut App) {
    app.register_type::<BlenderPlate>()
        .register_type::<Plate>()
        .add_observer(on_add_plate)
        .add_systems(PreUpdate, replace_blender_object::<BlenderPlate>);
}

/// Marker type for plate with door id
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BlenderPlate {
    level: Level,
}

impl BlenderObject for BlenderPlate {
    type BevyComponent = Plate;
    fn level(&self) -> Level {
        self.level
    }

    fn to_component(&self) -> Self::BevyComponent {
        Plate
    }
}

/// Plate for trickering events

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Plate;

fn on_add_plate(
    trigger: Trigger<OnAdd, Plate>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add mesh and animation junk
    commands
        .entity(trigger.target())
        .insert((
            Sensor,
            RigidBody::Kinematic,
            ElevatorKey::default(),
            Mesh3d(meshes.add(Cuboid::new(2., 0.8, 2.))),
            MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::BLACK))),
            CollisionEventsEnabled,
            ColliderConstructor::Cuboid {
                x_length: 2.,
                y_length: 0.8,
                z_length: 2.,
            },
        ))
        .observe(press_plate)
        .observe(leave_plate);
}

fn press_plate(
    trigger: Trigger<OnCollisionStart>,
    plates: Query<(&Plate, &KeyFor)>,
    mut commands: Commands,
    mut player: Query<Entity, With<Player>>,
) {
    let (plate, elevator) = plates.get(trigger.target()).unwrap();
    //only if the trigger was the human
    let event = trigger.event();
    //dont use event.body,
    let Ok(player) = player.get_mut(event.collider) else {
        return;
    };
    info!("Plate Stepped on!");
    commands.entity(elevator.0).trigger(Interact::dont_record());
}

fn leave_plate(
    trigger: Trigger<OnCollisionEnd>,
    plates: Query<&Plate>,
    mut commands: Commands,
    mut player: Query<Entity, With<Player>>,
) {
    let plate = plates.get(trigger.target()).unwrap();
    //only if the trigger was the human
    let event = trigger.event();
    //dont use event.body,
    let Ok(player) = player.get_mut(event.collider) else {
        return;
    };
    info!("Plate Stepped Off!");
}
