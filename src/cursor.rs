use bevy::prelude::*;

use crate::camera_controller::CameraMoved;
use crate::CAMERA_SIZE;

const CURSOR_SPRITE: &str = "cursor.png";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CursorState {
    GameCursor,
    UICursor,
}

#[derive(Component)]
pub struct Cursor;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MousePos {
            x: 0.0,
            y: 0.0,
            screen_x: 0.0,
            screen_y: 0.0,
        })
        .add_startup_system(spawn_mouse_cursor)
        .add_system(update_mouse_pos.label(UpdatedMousePos).after(CameraMoved))
        .add_system(place_mouse_cursor.after(UpdatedMousePos));
    }
}

pub struct MousePos {
    pub x: f32,
    pub y: f32,
    #[allow(dead_code)]
    pub screen_x: f32,
    #[allow(dead_code)]
    pub screen_y: f32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, SystemLabel)]
pub struct UpdatedMousePos;

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

fn place_mouse_cursor(mut cursor: Query<&mut Transform, With<Cursor>>, mouse_pos: Res<MousePos>) {
    let mut cursor = cursor.single_mut();
    cursor.translation = Vec3::new(mouse_pos.x, mouse_pos.y, 0.1);
}

fn spawn_mouse_cursor(mut commands: Commands, assets: Res<AssetServer>) {
    let image = assets.load(CURSOR_SPRITE);

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::splat(0.0),
                scale: Vec3::splat(1.0 / 128.0),
                ..Default::default()
            },
            texture: image,
            ..Default::default()
        })
        .insert(Cursor)
        .insert(Name::new("Cursor"));
}
