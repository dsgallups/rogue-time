use avian3d::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<TimeBank>();

    app.add_observer(insert_timebank);
}

/// This is going to be something that gives time to the user
///
/// This is inserted in blender
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TimeBank {
    pub milliseconds: u32,
}

fn insert_timebank(trigger: Trigger<OnAdd, TimeBank>, mut commands: Commands) {
    error!("Inserting timebank");
    //let collider =
    //ColliderConstructorHierarchy::new(ColliderConstructor::ConvexDecompositionFromMesh);

    //let collider = Collider::cylinder(0.5, 50.);
    commands
        .entity(trigger.target())
        .insert((
            RigidBody::Static,
            Collider::cylinder(1., 2.),
            //collider,
            //Sensor,
            CollisionEventsEnabled,
            CollidingEntities::default(),
        ))
        .observe(collect_timebank);

    // commands
    //     .entity(trigger.target())
    //     .insert((CollisionEventsEnabled, CollidingEntities::default()))
    //     .observe(collect_timebank);
}

fn collect_timebank(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    transform: Query<&Transform>,
) {
    let loc = transform.get(trigger.target()).unwrap();
    error!("Collision on timebank detected!, transform: {loc:?}");

    //commands.entity(trigger.observer()).despawn();
}
