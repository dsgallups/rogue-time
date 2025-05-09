use crate::level::{Level, LevelOrigins};
use bevy::prelude::*;

pub trait BlenderObject: Component {
    type BevyComponent: Component;
    fn level(&self) -> Level;
    fn to_component(&self) -> Self::BevyComponent;
}

pub fn replace_blender_object<T: BlenderObject>(
    mut commands: Commands,
    blender_timebanks: Query<(Entity, &Transform, &T)>,
    level_origins: Res<LevelOrigins>,
) {
    for (entity, transform, timebank) in blender_timebanks {
        info!("added blender timebank");

        let origin = level_origins.get_spawn_point(timebank.level());
        let transform = transform.with_translation(transform.translation + origin);

        commands.entity(entity).despawn();
        commands.spawn((timebank.level(), timebank.to_component(), transform));
    }
}
