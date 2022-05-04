use bevy::prelude::*;

use bevy::render::camera::ScalingMode;
use bevy::sprite::{Material2dPlugin, MaterialMesh2dBundle};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use player::PlayerPlugin;

use self::camera_controller::CameraControllerPlugin;
use self::debug::DebugMaterial;
use self::player::Player;

mod camera_controller;
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
        .insert_resource(MousePos {
            x: 0.0,
            y: 0.0,
            screen_x: 0.0,
            screen_y: 0.0,
        })
        .add_startup_system(setup)
        .add_system(update_mouse_pos)
        .add_plugin(CameraControllerPlugin)
        .add_plugin(PlayerPlugin)
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
        material: materials.add(DebugMaterial { color: Color::RED }),
        transform: Transform::default(),
        ..Default::default()
    });
}

pub struct MousePos {
    x: f32,
    y: f32,
    #[allow(dead_code)]
    screen_x: f32,
    #[allow(dead_code)]
    screen_y: f32,
}

fn update_mouse_pos(
    windows: Res<Windows>,
    mut mouse_moved_event: EventReader<CursorMoved>,
    mut mouse_pos: ResMut<MousePos>,
    camera: Query<&Transform, With<Camera>>,
) {
    for ev in mouse_moved_event.iter() {
        let window = windows.get_primary().unwrap();
        let width = window.width();
        let height = window.height();
        let (mx, my) = (ev.position.x, ev.position.y);
        let mx = 2.0 * CAMERA_SIZE * (mx - width / 2.0) / height;
        let my = 2.0 * CAMERA_SIZE * (my - height / 2.0) / height;
        mouse_pos.screen_x = mx;
        mouse_pos.screen_y = my;
    }

    let camera_translation = camera.single().translation;

    mouse_pos.x = mouse_pos.screen_x + camera_translation.x;
    mouse_pos.y = mouse_pos.screen_y + camera_translation.y;
}
