use bevy::prelude::*;

use crate::player::Player;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(follow_player);
    }
}

fn follow_player(
    mut camera: Query<&mut Transform, With<Camera>>,
    player: Query<&mut Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera = camera.single_mut();
    let player = player.single();

    camera.translation = player.translation;
}
