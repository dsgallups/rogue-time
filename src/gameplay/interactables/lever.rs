use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};
mod animation;
use crate::gameplay::door::{Door, KeyFor};

pub fn plugin(app: &mut App) {
    app.register_type::<BlenderLever>()
        .register_type::<Lever>()
        .add_observer(on_add_lever)
        .add_plugins(animation::plugin)
        .add_systems(Update, on_add_blender_lever);
}

/// Marker type for lever with door id
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BlenderLever(u8);

/// Lever for trickering events
#[derive(Component, Reflect)]
#[require(Pickable)]
#[reflect(Component)]
pub struct Lever;

// TODO NEEDS TO RUN AFTER [`Door`]s ARE INSERTED
fn on_add_blender_lever(
    mut commands: Commands,
    blender_lever: Query<(Entity, &Transform, &BlenderLever)>,
    doors: Query<(Entity, &Door)>,
) {
    for (entity, transform, lever) in blender_lever.iter() {
        // Skip if the door ID is not the same as the lever ID
        let Some((door_entity, _)) = doors.iter().find(|(_, door)| door.0 == lever.0) else {
            continue;
        };

        // Despawn the original lever entity
        commands.entity(entity).despawn();

        // Spawn the new lever in relation to the matching door
        commands.entity(door_entity).with_related_entities(
            |door: &mut RelatedSpawnerCommands<KeyFor>| {
                door.spawn((Lever, *transform)).observe(flip_lever);
                info!("Spawned lever {}", lever.0);
            },
        );
    }
}

fn on_add_lever(trigger: Trigger<OnAdd, Lever>, mut commands: Commands) {
    // Add mesh and animation junk
    commands.entity(trigger.target());
}

fn flip_lever(_trigger: Trigger<Pointer<Click>>) {
    info!("Lever clicked!");
}
