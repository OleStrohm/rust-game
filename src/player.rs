use crate::cursor::{Cursor, CursorState, MousePos};
use crate::debug::{DebugCircle, DebugLineData, DebugRect};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiContext;
use bevy_inspector_egui::Inspectable;
use std::f32::consts::PI;
use std::time::Duration;

const COMPASS_SPRITE: &str = "compass.png";
const LASER_SPRITE: &str = "laser.png";
const ROCK_SPRITE: &str = "rock.png";

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
}

#[derive(Component)]
pub struct Laser {
    lifetime: Duration,
    origin: Duration,
}

pub struct PlayerPlugin;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, SystemLabel)]
pub struct PlayerMoved;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(CursorState::GameCursor)
            .add_system_set(
                SystemSet::on_enter(CursorState::GameCursor).with_system(to_game_cursor),
            )
            .add_system_set(SystemSet::on_enter(CursorState::UICursor).with_system(to_ui_cursor))
            .add_system_set(
                SystemSet::on_update(CursorState::UICursor).with_system(change_cursor_state),
            )
            .add_system_set(
                SystemSet::on_update(CursorState::GameCursor)
                    .with_system(shoot)
                    .with_system(change_cursor_state),
            )
            .add_startup_system(spawn_player)
            .add_startup_system(spawn_some_rocks)
            .add_system(move_player.label(PlayerMoved))
            .add_system(spawn_some_rocks_on_space)
            .add_system(on_hit_rock)
            .add_system(move_laser);
    }
}

#[derive(Component)]
pub struct Rock;

fn spawn_some_rocks_on_space(
    commands: Commands,
    assets: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        spawn_some_rocks(commands, assets);
    }
}

fn spawn_some_rocks(mut commands: Commands, assets: Res<AssetServer>) {
    let image = assets.load(ROCK_SPRITE);

    for _ in 0..10 {
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        6. * rand::random::<f32>() - 3.,
                        6. * rand::random::<f32>() - 3.,
                        0.1,
                    ),
                    scale: Vec3::splat(0.25),
                    ..Default::default()
                },
                texture: image.clone(),
                sprite: Sprite {
                    custom_size: Some((1.0, 1.0).into()),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Rock)
            .insert(DebugCircle {
                color: Color::BLUE,
                radius: 1.0 / 4.0,
            })
            .insert(DebugRect {
                color: Color::GREEN,
                rotation: 0.0,
                size: (1.0 / 4.0, 1.0 / 4.0).into(),
            })
            .insert(Name::new("Rock"));
    }
}

fn on_hit_rock(
    mut commands: Commands,
    lasers: Query<(Entity, &Transform), With<Laser>>,
    rocks: Query<(Entity, &Transform), With<Rock>>,
) {
    for (laser_ent, laser_tr) in lasers.iter() {
        for (rock_ent, rock_tr) in rocks.iter() {
            if laser_tr.translation.xy().distance(rock_tr.translation.xy()) <= 0.2 {
                commands.entity(laser_ent).despawn_recursive();
                commands.entity(rock_ent).despawn_recursive();
            }
        }
    }
}

fn change_cursor_state(
    mut cursor_state: ResMut<State<CursorState>>,
    mut egui_context: ResMut<EguiContext>,
) {
    let new_state = if egui_context.ctx_mut().is_pointer_over_area() {
        CursorState::UICursor
    } else {
        CursorState::GameCursor
    };
    if &new_state != cursor_state.current() {
        cursor_state.set(new_state).unwrap();
    }
}

pub fn to_game_cursor(
    mut cursor_query: Query<&mut Visibility, With<Cursor>>,
    mut windows: ResMut<Windows>,
) {
    cursor_query.single_mut().is_visible = true;
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(false);
}

pub fn to_ui_cursor(
    mut cursor_query: Query<&mut Visibility, With<Cursor>>,
    mut windows: ResMut<Windows>,
) {
    cursor_query.single_mut().is_visible = false;
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(true);
}

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let image = assets.load(COMPASS_SPRITE);

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., 0.9),
                scale: Vec3::splat(1.0 / 16.0),
                ..Default::default()
            },
            texture: image,
            ..Default::default()
        })
        .insert(Player { speed: 4.0 })
        .insert(Name::new("Player"))
        .insert(DebugCircle {
            color: Color::GREEN,
            radius: 0.5,
        })
        .insert(DebugRect {
            color: Color::RED,
            rotation: 0.0,
            size: (2.0, 1.0).into(),
        });
}

fn shoot(
    mut commands: Commands,
    assets: Res<AssetServer>,
    query: Query<&Transform, With<Player>>,
    mouse: Res<Input<MouseButton>>,
    time: Res<Time>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let transform = query.single();

        let laser_image = assets.load(LASER_SPRITE);

        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: transform.translation.xy().extend(0.),
                    rotation: transform.rotation,
                    scale: Vec3::splat(1.0 / 64.0),
                },
                texture: laser_image,
                ..Default::default()
            })
            .insert(Laser {
                lifetime: Duration::from_secs(1),
                origin: time.time_since_startup(),
            })
            .insert(Name::new("Player Laser"))
            .insert(DebugCircle {
                color: Color::BLUE,
                radius: 0.2,
            });
    }
}

fn move_laser(
    mut commands: Commands,
    mut lasers: Query<(Entity, &Laser, &mut Transform)>,
    time: Res<Time>,
) {
    let speed = 10. * time.delta_seconds();
    for (entity, laser, mut transform) in lasers.iter_mut() {
        let direction =
            (transform.rotation * Quat::from_rotation_z(PI / 2.0)).mul_vec3(Vec3::new(1., 0., 0.));
        transform.translation += speed * direction;
        if laser.lifetime + laser.origin <= time.time_since_startup() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn move_player(
    mut query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    mouse_pos: Res<MousePos>,
    time: Res<Time>,
) {
    let (player, mut transform) = query.single_mut();
    let speed = player.speed * time.delta_seconds();
    if keyboard.pressed(KeyCode::W) {
        transform.translation.y += speed;
    }
    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= speed;
    }
    if keyboard.pressed(KeyCode::A) {
        transform.translation.x -= speed;
    }
    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += speed;
    }

    let (x, y, _) = transform.translation.into();
    let (dx, dy) = (mouse_pos.x - x, mouse_pos.y - y);
    let angle = dy.atan2(dx);
    transform.rotation = Quat::from_rotation_z(angle - PI / 2.0);
}
