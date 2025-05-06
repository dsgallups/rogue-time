use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use default_input::DefaultInputContext;

//use crate::third_party::avian3d::CollisionLayer;

use super::stopwatch::StopwatchTimer;

pub mod camera;
mod default_input;
pub mod movement;

pub fn plugin(app: &mut App) {
    app.register_type::<Player>();

    app.add_plugins((camera::plugin, default_input::plugin, movement::plugin));
    app.add_observer(setup_player);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player;

/// The radius of the player character's capsule.
pub(crate) const PLAYER_RADIUS: f32 = 0.5;
/// The length of the player character's capsule. Note that
const PLAYER_CAPSULE_LENGTH: f32 = 1.0;

/// The total height of the player character's capsule. A capsule's height is `length + 2 * radius`.
const PLAYER_HEIGHT: f32 = PLAYER_CAPSULE_LENGTH + 2.0 * PLAYER_RADIUS;
/// The half height of the player character's capsule is the distance between the character's center and the lowest point of its collider.
const PLAYER_HALF_HEIGHT: f32 = PLAYER_HEIGHT / 2.0;

/// The height used for the player's floating character controller.
///
/// Such a controller works by keeping the character itself at a more-or-less constant height above the ground by
/// using a spring. It's important to make sure that this floating height is greater (even if by little) than the half height.
///
/// In this case, we use 30 cm of padding to make the player float nicely up stairs.
const PLAYER_FLOAT_HEIGHT: f32 = PLAYER_HALF_HEIGHT + 0.01;

fn setup_player(trigger: Trigger<OnAdd, Player>, mut commands: Commands) {
    let mut stopwatch_timer = StopwatchTimer::default();
    stopwatch_timer.pause();
    commands.entity(trigger.target()).insert((
        RigidBody::Dynamic,
        Actions::<DefaultInputContext>::default(),
        Transform::from_xyz(2.0, 15., 2.0),
        // The player character needs to be configured as a dynamic rigid body of the physics
        // engine.
        Collider::capsule(PLAYER_RADIUS, PLAYER_CAPSULE_LENGTH),
        // This is Tnua's interface component.
        TnuaController::default(),
        // A sensor shape is not strictly necessary, but without it we'll get weird results.
        TnuaAvian3dSensorShape(Collider::cylinder(PLAYER_RADIUS - 0.01, 0.0)),
        // Tnua can fix the rotation, but the character will still get rotated before it can do so.
        // By locking the rotation we can prevent this.
        LockedAxes::ROTATION_LOCKED,
        // Movement feels nicer without friction.
        Friction {
            dynamic_coefficient: 0.0,
            static_coefficient: 0.0,
            combine_rule: CoefficientCombine::Multiply,
        },
        stopwatch_timer,
        ColliderDensity(100.0),
        CollisionEventsEnabled,
        CollidingEntities::default(), //CollisionLayers::new(CollisionLayer::Character, LayerMask::ALL),
                                      //TnuaAnimatingState::<PlayerAnimationState>::default(),
                                      //PlayerLandmassCharacter(player_character),
    ));
    //.observe(setup_player_animations);
}
