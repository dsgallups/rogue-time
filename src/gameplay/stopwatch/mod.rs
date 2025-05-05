use std::time::Duration;

use bevy::prelude::*;

use super::{GameSet, animation::AnimationPlayerAncestor};

mod animation;

pub fn plugin(app: &mut App) {
    app.register_type::<Stopwatch>();

    app.add_plugins(animation::plugin);

    app.add_observer(on_stopwatch_spawn);

    app.add_systems(Update, tick_stopwatch.in_set(GameSet::TickTimers));
}

#[derive(Component)]
pub struct StopwatchTimer(pub Timer);

impl StopwatchTimer {
    pub fn new(initial_time: Duration) -> Self {
        let timer = Timer::new(initial_time, TimerMode::Once);
        Self(timer)
    }
    pub fn pause(&mut self) {
        self.0.pause();
    }
    pub fn unpause(&mut self) {
        self.0.unpause();
    }
    pub fn add_time(&mut self, time: Duration) {
        let current_duration = self.0.duration();

        let new_duration = current_duration + time;
        self.0.set_duration(new_duration);
    }
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

fn tick_stopwatch(mut stopwatches: Query<&mut StopwatchTimer>, time: Res<Time>) {
    for mut stopwatch in &mut stopwatches {
        stopwatch.0.tick(time.delta());
    }
}
