#![doc = r#"
This is going to be a top-down camera.


What we will do here is PRETEND like this is a 2d game,
when we're going to actually be using 3d models and meshes.

The advantage of this is that we can switch to 3d if necessary in the future.

"#]

use bevy::prelude::*;

use crate::screens::Screen;

pub fn plugin(app: &mut App) {
    app.register_type::<ThirdPersonCamera>()
        .register_type::<CameraTarget>();

    app.add_systems(OnEnter(Screen::Gameplay), spawn_cam)
        .add_systems(PostUpdate, smooth_follow.run_if(in_state(Screen::Gameplay)));
}

#[derive(Component, Clone, Copy, Reflect)]
struct ThirdPersonCamera {
    offset_x: f32,
    // this is the height
    offset_y: f32,

    // this is the forward/backward offset from the target.
    //
    // dont forget! -Z is forward (so this is most likely positive)
    offset_z: f32,
}
impl Default for ThirdPersonCamera {
    fn default() -> Self {
        Self {
            offset_x: 0.,
            offset_y: 2.,
            offset_z: 5.,
        }
    }
}

/// The camera *should* follow this entity
#[derive(Component, Reflect)]
pub struct CameraTarget {
    follow_rotation: bool,

    /// So even though the camera has an offset_y,
    /// sometimes, the target's transform will be on the floor.
    /// So this sets where the camera is going to look. It shouldn't
    /// be looking at the the target's feet. not into that
    ///
    /// i.e. I have a human model from a glb scene.
    ///
    /// Well, the root transform is probably going to be at y: 0.
    /// So I will set this value to be whatever the half height is manually.
    pub target_y_offset: f32,

    // how quickly should the camera snap to this target's location
    transform_decay_rate: f32,
    // how quickly should the camera rotate from its orientation respect to the target's?
    rotation_decay_rate: f32,
}
#[allow(dead_code)]
impl CameraTarget {
    /// This means that we will rotate to "sit behind" the target, respecting their rotation.
    pub fn follow_rotation() -> Self {
        Self {
            follow_rotation: true,
            target_y_offset: 0.,
            transform_decay_rate: 2.,
            rotation_decay_rate: 2.,
        }
    }
    pub fn with_transform_decay_rate(mut self, decay_rate: f32) -> Self {
        self.transform_decay_rate = decay_rate;
        self
    }
    pub fn with_rotation_decay_rate(mut self, decay_rate: f32) -> Self {
        self.rotation_decay_rate = decay_rate;
        self
    }
    pub fn with_y_offset(mut self, y_offset: f32) -> Self {
        self.target_y_offset = y_offset;
        self
    }
    #[allow(dead_code)]
    pub fn dont_follow_rotation() -> Self {
        Self {
            follow_rotation: false,
            target_y_offset: 0.,
            transform_decay_rate: 2.,
            rotation_decay_rate: 2.,
        }
    }
}

fn spawn_cam(mut commands: Commands) {
    let topdown = ThirdPersonCamera::default();
    commands.spawn((
        Camera3d::default(),
        StateScoped(Screen::Gameplay),
        topdown,
        Transform::from_xyz(topdown.offset_x, topdown.offset_y, topdown.offset_z)
            .looking_at(Vec3::ZERO, Vec3::Y),
        Projection::Perspective(PerspectiveProjection {
            fov: 90.0_f32.to_radians(),
            ..default()
        }),
    ));
}

/// Update the camera position by tracking the player.
fn smooth_follow(
    mut camera: Query<(&mut Transform, &ThirdPersonCamera), Without<CameraTarget>>,
    player: Query<(&Transform, &CameraTarget)>,
    time: Res<Time>,
) {
    let Ok((mut cam_transl, cam)) = camera.single_mut() else {
        panic!("no td camera");
    };
    let Ok((target_transform, target)) = player.single() else {
        // no target
        return;
    };
    // we rotate with the target
    if target.follow_rotation {
        // let (yaw, pitch, roll) = player.rotation.to_euler(EulerRot::YXZ);
        // let rotation =
        //     Quat::from_euler(EulerRot::YXZ, yaw, pitch + target.camera_delta_pitch, roll);

        // Calculate the desired camera translation based, radius, and xy_offset
        let rotation_matrix = Mat3::from_quat(target_transform.rotation);

        // get the offset
        let offset = rotation_matrix.mul_vec3(Vec3::new(cam.offset_x, cam.offset_y, cam.offset_z));

        let mut desired_translation = offset - rotation_matrix.mul_vec3(Vec3::new(0.0, 0.0, 1.));
        desired_translation.y = desired_translation.y.max(0.);

        let desired_translation = target_transform.translation + desired_translation;
        let look_at = target_transform.translation + Vec3::new(0., target.target_y_offset, 0.);
        let desired_transform = Transform::from_xyz(
            desired_translation.x,
            desired_translation.y,
            desired_translation.z,
        )
        .looking_at(look_at, Vec3::Y);

        // Applies a smooth effect to camera movement using stable interpolation
        // between the camera position and the player position on the x and y axes.
        cam_transl.translation.smooth_nudge(
            &desired_transform.translation,
            target.transform_decay_rate,
            time.delta_secs(),
        );

        cam_transl.rotation.smooth_nudge(
            &desired_transform.rotation,
            target.rotation_decay_rate,
            time.delta_secs(),
        );
    } else {
        // we maintain our current rotation, so that means the target's offset_y prop doesn't apply.
        //
        // yes that condition is invariant, but it's annoying to make that an enum
        let Vec3 { x, z, .. } = target_transform.translation;
        let direction = Vec3::new(x + cam.offset_x, cam.offset_y, z + cam.offset_z);
        // Applies a smooth effect to camera movement using stable interpolation
        // between the camera position and the player position on the x and y axes.
        cam_transl.translation.smooth_nudge(
            &direction,
            target.transform_decay_rate,
            time.delta_secs(),
        );
    }
}
