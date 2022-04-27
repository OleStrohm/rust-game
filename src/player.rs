use std::f32::consts::PI;
use std::time::Duration;
use bevy::prelude::*;
use crate::MousePos;

const COMPASS_SPRITE: &str = "compass.png";
const LASER_SPRITE: &str = "laser.png";

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Laser {
    lifetime: Duration,
    origin: Duration,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_compass)
            .add_system(move_player)
            .add_system(shoot)
            .add_system(move_laser);
    }
}

fn spawn_compass(mut commands: Commands, assets: Res<AssetServer>) {
    let image = assets.load(COMPASS_SPRITE);

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::splat(0.0),
                scale: Vec3::splat(1.0 / 32.0),
                rotation: Quat::from_rotation_z(3.1),
            },
            texture: image,
            ..Default::default()
        })
        .insert(Player);
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
                    translation: transform.translation,
                    rotation: transform.rotation,
                    scale: Vec3::splat(1.0 / 64.0),
                },
                texture: laser_image,
                ..Default::default()
            })
            .insert(Laser {
                lifetime: Duration::from_secs(1),
                origin: time.time_since_startup(),
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

fn move_player(
    mut query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    mouse_pos: Res<MousePos>,
    time: Res<Time>,
) {
    let (_player, mut transform) = query.single_mut();
    let speed = 2.0 * time.delta_seconds();
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
