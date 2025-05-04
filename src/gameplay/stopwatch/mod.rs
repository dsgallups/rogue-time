use bevy::prelude::*;

use super::animation::AnimationPlayerAncestor;

mod animation;

pub fn plugin(app: &mut App) {
    app.register_type::<Stopwatch>();

    app.add_plugins(animation::plugin);

    app.add_observer(on_stopwatch_spawn);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Stopwatch;

// this *probably* triggers before `OnAdd, SceneInstanceReady`
//
// TODO: we should add this to the glb directly with skein
fn on_stopwatch_spawn(trigger: Trigger<OnAdd, Stopwatch>, mut commands: Commands) {
    // this will then trigger the animation plugin
    //
    // which in turn triggers the `setup_stopwatch_animation` system in this
    // module's animation plugin
    commands
        .entity(trigger.target())
        .observe(animation::setup_stopwatch_animation)
        .insert(AnimationPlayerAncestor);
}
