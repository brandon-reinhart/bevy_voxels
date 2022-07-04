use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

const CAMERA_SPEED:f32 = 10.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(update)
        ;
    }
}

fn setup(
    mut commands: Commands
) {
    commands.spawn_bundle(
        PerspectiveCameraBundle {
            transform: Transform::from_xyz(0., 0., -10.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        }
    );
}

fn update(
    time: Res<Time>,
    mut mouse_motion: EventReader<MouseMotion>,
    keys: Res<Input<KeyCode>>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    if query.is_empty() {
        return
    }

    let mut camera_transform = query.single_mut();
    let delta_time = time.delta_seconds();

    if keys.pressed(KeyCode::W) {
        let forward = camera_transform.forward() * CAMERA_SPEED * delta_time;
        camera_transform.translation += forward;
    }

    if keys.pressed(KeyCode::A) {
        let left = camera_transform.left() * CAMERA_SPEED * delta_time;
        camera_transform.translation += left;
    }

    if keys.pressed(KeyCode::S) {
        let back = camera_transform.back() * CAMERA_SPEED * delta_time;
        camera_transform.translation += back;
    }

    if keys.pressed(KeyCode::D) {
        let right = camera_transform.right() * CAMERA_SPEED * delta_time;
        camera_transform.translation += right;
    }

    if mouse_buttons.pressed(MouseButton::Right) {
        for motion in mouse_motion.iter() {
            let yaw = Quat::from_rotation_y(-motion.delta.x * delta_time);
            let pitch = Quat::from_rotation_x(-motion.delta.y * delta_time);
            camera_transform.rotation = yaw * camera_transform.rotation;
            camera_transform.rotation = camera_transform.rotation * pitch;
        }
    }

}

