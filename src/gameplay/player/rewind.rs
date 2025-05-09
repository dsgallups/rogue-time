use bevy::prelude::*;

use crate::gameplay::{GameSet, GameState, room::StartCountdown};

use super::{Player, camera::PlayerCamera, movement::MovementDisabled};

/// How many times per second we record the player position
pub const LOG_FREQUENCY: f32 = 8.;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, rewind_input.in_set(GameSet::RecordInput))
        .add_observer(reset_logtimer_on_rewind)
        .add_observer(start_log_on_new_room)
        .insert_resource(LogPeriod(Timer::from_seconds(
            1.0 / LOG_FREQUENCY,
            TimerMode::Repeating,
        )))
        .add_systems(Update, tick_log_timer.in_set(GameSet::TickTimers))
        .add_systems(
            Update,
            record_movements.in_set(GameSet::RecordInput).run_if(
                in_state(GameState::Playing)
                    .and(resource_exists::<MovementLog>)
                    .and(|timer: Res<LogPeriod>| timer.0.just_finished()),
            ),
        )
        .add_systems(
            Update,
            play_logged_recording
                .in_set(GameSet::RecordInput)
                .run_if(in_state(GameState::Rewinding).and(resource_exists::<MovementLog>)),
        )
        .add_observer(handle_rewind_event);
}

#[derive(Event)]
pub enum RewindAnimation {
    Start,
    End,
}

#[derive(Component)]
pub struct CanRewind;

/// Timer for triggering [`Player`] snapshots
#[derive(Resource)]
pub struct LogPeriod(Timer);

fn tick_log_timer(time: Res<Time>, mut timer: ResMut<LogPeriod>) {
    timer.0.tick(time.delta());
}

/// Stores [`Player`] movement From start of a level
#[derive(Resource, Default)]
pub struct MovementLog {
    player: Vec<Transform>,
    camera: Vec<Transform>,
}

/// Adds Movement Log when room starts
fn start_log_on_new_room(
    _trigger: Trigger<StartCountdown>,
    mut commands: Commands,
    mut timer: ResMut<LogPeriod>,
) {
    timer.0.reset();
    timer.0.unpause();
    commands.init_resource::<MovementLog>();
}

// in theory, we could make this an observer attached to the player
// on the collect_timebank fn but fk it
fn rewind_input(
    mut commands: Commands,
    has_rewind: Query<Entity, (With<Player>, With<CanRewind>, Without<MovementDisabled>)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok(entity) = has_rewind.single() else {
        //can't rewind
        return;
    };

    if !keys.just_pressed(KeyCode::KeyE) {
        return;
    }
    commands.entity(entity).remove::<CanRewind>();

    commands.trigger(RewindAnimation::Start);
}

/// Sets state depending on rewind trigger
fn handle_rewind_event(
    trigger: Trigger<RewindAnimation>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    match trigger.event() {
        RewindAnimation::Start => next_state.set(GameState::Rewinding),
        RewindAnimation::End => next_state.set(GameState::Playing),
    }
}

fn reset_logtimer_on_rewind(_trigger: Trigger<RewindAnimation>, mut timer: ResMut<LogPeriod>) {
    timer.0.pause();
    timer.0.reset();
}

/// reads out [`MovementLog`] LIFO fashion
fn play_logged_recording(
    mut commands: Commands,
    camera: Query<Entity, With<PlayerCamera>>,
    player: Query<Entity, With<Player>>,
    mut log: ResMut<MovementLog>,
) {
    let (Ok(camera), Ok(player)) = (camera.single(), player.single()) else {
        return;
    };

    let (Some(player_transform), Some(camera_transfrom)) = (log.player.pop(), log.camera.pop())
    else {
        commands.trigger(RewindAnimation::End);
        return;
    };

    commands.entity(camera).insert(camera_transfrom);
    commands.entity(player).insert(player_transform);
}

/// Record movement to [`MovementLog`] stack when in [`GameState::Playing`] during a level
fn record_movements(
    camera_transform: Query<&Transform, With<PlayerCamera>>,
    player_transform: Query<&Transform, With<Player>>,
    mut log: ResMut<MovementLog>,
) {
    let (Ok(camera_transform), Ok(player_transform)) =
        (camera_transform.single(), player_transform.single())
    else {
        return;
    };

    log.player.push(*player_transform);
    log.camera.push(*camera_transform);
}
