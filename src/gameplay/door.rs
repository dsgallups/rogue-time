use bevy::prelude::*;

mod animation;

pub fn plugin(app: &mut App) {
    app.register_type::<BlenderDoor>()
        .register_type::<Door>()
        .register_type::<KeyFor>()
        .register_type::<Unlockers>()
        .add_plugins(animation::plugin)
        .add_observer(on_add_door)
        .add_systems(PreUpdate, on_add_blender_door);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct BlenderDoor(u8);

fn on_add_blender_door(
    mut commands: Commands,
    blender_door: Query<(Entity, &Transform, &BlenderDoor)>,
) {
    for (entity, transform, door) in blender_door {
        // we are going to take this thing,
        // remove it from the scene entirely,
        // and then construct it ourselves.
        commands.entity(entity).despawn();
        commands.spawn((Door(door.0), *transform));
        info!("Spawned Door")
    }
}
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Door(pub u8);

fn on_add_door(trigger: Trigger<OnAdd, Door>, mut commands: Commands) {
    commands.entity(trigger.target()).observe(try_open_door);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = Unlockers )]
pub struct KeyFor(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship=KeyFor, linked_spawn)]
pub struct Unlockers(Vec<Entity>);

// Should open the door with the captured entity
#[derive(Event)]
struct OpenDoor(Entity);

fn try_open_door(_trigger: Trigger<Pointer<Click>>) {
    info!("Door Triggered")
}
