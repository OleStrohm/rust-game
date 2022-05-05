use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

use crate::player::{Player, PlayerMoved};

pub struct CameraControllerPlugin;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, SystemLabel)]
pub struct CameraMoved;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(follow_player.label(CameraMoved).after(PlayerMoved));
    }
}

pub fn follow_player(
    mut camera: Query<&mut Transform, With<Camera>>,
    player: Query<&mut Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera = camera.single_mut();
    let player = player.single();

    camera.translation = player.translation.xy().extend(999.9);
}
