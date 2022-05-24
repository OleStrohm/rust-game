use bevy::prelude::*;

use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use player::PlayerPlugin;

use self::camera_controller::CameraControllerPlugin;
use self::cursor::CursorPlugin;
use self::debug::DebugPlugin;
use self::player::Player;

mod camera_controller;
mod cursor;
mod debug;
mod player;
mod tilemap;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.35, 0.1, 0.7)))
        .insert_resource(WindowDescriptor {
            title: "Making a Game in Rust".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<Player>()
        .add_plugin(CameraControllerPlugin)
        .add_plugin(CursorPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut windows: ResMut<Windows>) {
    // position window
    let window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(
        (2560 - window.width() as i32) / 2,
        (1440 - window.height() as i32) / 2,
    ));

    window.set_cursor_visibility(false);
}
