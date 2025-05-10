use bevy::prelude::*;

use crate::gameplay::{
    GameSet, GameState,
    room::{NewRoom, RoomStarted},
};

use super::{Player, TeleportTo, camera::PlayerCamera, movement::MovementDisabled};

/// How many times per second we record the player position
pub const LOG_FREQUENCY: f32 = 8.;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, rewind_input.in_set(GameSet::RecordInput))
        .add_observer(on_new_room)
        .add_observer(on_room_start)
        .init_resource::<MovementLog>()
        .add_systems(Update, tick_log_timer.in_set(GameSet::TickTimers))
        .add_systems(
            Update,
            record_movements
                .in_set(GameSet::RecordInput)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            play_logged_recording
                .in_set(GameSet::RecordInput)
                .run_if(in_state(GameState::Rewinding)),
        )
        .add_observer(on_start_rewind)
        .add_observer(clear_rewind_on_teleport)
        .add_observer(on_end_rewind);
}

#[derive(Event)]
pub struct StartRewind;

#[derive(Event)]
pub struct EndRewind;

#[derive(Component)]
pub struct CanRewind;

fn tick_log_timer(time: Res<Time>, mut log: ResMut<MovementLog>) {
    log.timer.tick(time.delta());
}

/// Stores [`Player`] movement From start of a level and timer
#[derive(Resource)]
pub struct MovementLog {
    timer: Timer,
    player: Vec<Transform>,
    camera: Vec<Transform>,
}

impl Default for MovementLog {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0 / LOG_FREQUENCY, TimerMode::Repeating),
            player: Vec::new(),
            camera: Vec::new(),
        }
    }
}

impl MovementLog {
    pub fn reset(&mut self) {
        self.player = Vec::new();
        self.camera = Vec::new();
    }
}

fn on_new_room(
    _trigger: Trigger<NewRoom>,
    mut commands: Commands,
    player: Query<Entity, With<CanRewind>>,
    mut log: ResMut<MovementLog>,
) {
    for player in player {
        commands.entity(player).remove::<CanRewind>();
    }

    log.timer.reset();
    log.timer.pause();
    log.reset();
}

/// Adds Movement Log when room starts
fn on_room_start(_trigger: Trigger<RoomStarted>, mut log: ResMut<MovementLog>) {
    log.timer.unpause();
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

    commands.trigger(StartRewind);
}

/// Sets state depending on rewind trigger
fn on_start_rewind(
    _trigger: Trigger<StartRewind>,
    mut next_state: ResMut<NextState<GameState>>,
    mut log: ResMut<MovementLog>,
) {
    next_state.set(GameState::Rewinding);
    log.timer.pause();
    log.timer.reset();
}

/// Sets state depending on rewind trigger
fn on_end_rewind(
    _trigger: Trigger<EndRewind>,
    mut next_state: ResMut<NextState<GameState>>,
    mut log: ResMut<MovementLog>,
) {
    next_state.set(GameState::Playing);
    log.timer.unpause();
}

/// reads out [`MovementLog`] LIFO fashion
fn play_logged_recording(
    mut commands: Commands,
    mut camera: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    mut player: Query<&mut Transform, (With<Player>, Without<PlayerCamera>)>,
    mut log: ResMut<MovementLog>,
) {
    let (Ok(mut camera), Ok(mut player)) = (camera.single_mut(), player.single_mut()) else {
        return;
    };

    let (Some(player_transform), Some(camera_transfrom)) = (log.player.pop(), log.camera.pop())
    else {
        commands.trigger(EndRewind);
        return;
    };
    *camera = camera_transfrom;
    *player = player_transform;
}

/// Record movement to [`MovementLog`] stack when in [`GameState::Playing`] during a level
fn record_movements(
    camera_transform: Query<&Transform, With<PlayerCamera>>,
    player_transform: Query<&Transform, With<Player>>,
    mut log: ResMut<MovementLog>,
) {
    if !log.timer.just_finished() {
        return;
    }
    let (Ok(camera_transform), Ok(player_transform)) =
        (camera_transform.single(), player_transform.single())
    else {
        return;
    };

    log.player.push(*player_transform);
    log.camera.push(*camera_transform);
}

fn clear_rewind_on_teleport(_trigger: Trigger<TeleportTo>, mut log: ResMut<MovementLog>) {
    log.player.clear();
    log.camera.clear();
    log.timer.reset();
    log.timer.pause();
}
