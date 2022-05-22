use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use crate::player::{Player, PlayerMoved};

pub const CAMERA_SIZE: f32 = 4.0;

pub struct CameraControllerPlugin;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, SystemLabel)]
pub struct CameraMoved;

#[derive(Component)]
pub struct CameraFollower;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_system(follow_player.label(CameraMoved).after(PlayerMoved))
            .add_system(follow_camera.after(CameraMoved));
    }
}

fn spawn_camera(mut commands: Commands, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    let aspect_ratio = window.width() / window.height();

    // Create camera
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.top = CAMERA_SIZE;
    camera.orthographic_projection.bottom = -CAMERA_SIZE;
    camera.orthographic_projection.right = CAMERA_SIZE * aspect_ratio;
    camera.orthographic_projection.left = -CAMERA_SIZE * aspect_ratio;
    camera.orthographic_projection.scaling_mode = ScalingMode::None;
    commands
        .spawn_bundle(camera)
        .insert(Name::new("Game Camera"));
}

pub fn follow_player(
    mut camera: Query<&mut Transform, With<Camera>>,
    player: Query<&mut Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera = camera.single_mut();
    let player = player.single();

    camera.translation = player.translation.xy().extend(999.9);
}

fn follow_camera(
    mut followers: Query<&mut Transform, With<CameraFollower>>,
    camera: Query<&Transform, (With<Camera>, Without<CameraFollower>)>,
) {
    let camera = camera.single();

    for mut follower in followers.iter_mut() {
        follower.translation.x = camera.translation.x;
        follower.translation.y = camera.translation.y;
    }
}
