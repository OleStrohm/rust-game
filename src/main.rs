use bevy::prelude::*;

use bevy::render::camera::ScalingMode;
use bevy::sprite::{Material2dPlugin, MaterialMesh2dBundle};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use player::PlayerPlugin;

use self::camera_controller::CameraControllerPlugin;
use self::cursor::CursorPlugin;
use self::debug::DebugMaterial;
use self::player::Player;

mod camera_controller;
mod cursor;
mod debug;
mod player;

const CAMERA_SIZE: f32 = 3.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.35, 0.1, 0.7)))
        .insert_resource(WindowDescriptor {
            title: "Rust game!".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(Material2dPlugin::<DebugMaterial>::default())
        .register_inspectable::<Player>()
        .add_plugin(CameraControllerPlugin)
        .add_plugin(CursorPlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<DebugMaterial>>,
) {
    // position window
    let window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(
        (2560 - window.width() as i32) / 2,
        (1440 - window.height() as i32) / 2,
    ));
    let aspect_ratio = window.width() / window.height();

    window.set_cursor_visibility(false);

    // Create camera
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.top = CAMERA_SIZE;
    camera.orthographic_projection.bottom = -CAMERA_SIZE;
    camera.orthographic_projection.right = CAMERA_SIZE * aspect_ratio;
    camera.orthographic_projection.left = -CAMERA_SIZE * aspect_ratio;
    camera.orthographic_projection.scaling_mode = ScalingMode::None;
    commands.spawn_bundle(camera);

    commands.spawn().insert_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        material: materials.add(DebugMaterial {
            color: Color::RED,
            radius: 10.0,
        }),
        transform: Transform::default(),
        ..Default::default()
    });
}
