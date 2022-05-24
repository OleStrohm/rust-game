use std::mem;

use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_inspector_egui::Inspectable;

use crate::debug::DebugRect;
use crate::player::Player;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileMap>()
            .init_resource::<TileSprites>()
            .add_startup_system(spawn_tiles)
            .add_system(update_tiles)
            .add_system(red_if_occupied);
    }
}

#[derive(Component, Inspectable, Clone, Hash, PartialEq, Eq)]
pub enum TileKind {
    Stone,
    Water,
    Grass,
    Wall,
}

impl TileKind {
    fn get_sprite(&self) -> &'static str {
        match self {
            TileKind::Grass => "grass.png",
            TileKind::Stone => "stone.png",
            _ => "grass.png",
        }
    }
}

struct TileSprites {
    sprites: HashMap<TileKind, Handle<Image>>,
}

impl FromWorld for TileSprites {
    fn from_world(world: &mut World) -> Self {
        Self {
            sprites: [TileKind::Stone, TileKind::Grass]
                .into_iter()
                .map(|kind| {
                    (
                        kind.clone(),
                        world.resource::<AssetServer>().load(kind.get_sprite()),
                    )
                })
                .collect(),
        }
    }
}

pub struct Tile {
    pos: IVec2,
    kind: TileKind,
}

#[derive(Default)]
pub struct TileMap {
    tiles: Vec<Entity>,
    to_be_added: Vec<Tile>,
}

impl TileMap {
    pub fn set_tile(&mut self, x: i32, y: i32, kind: TileKind) {
        self.to_be_added.push(Tile {
            pos: IVec2::new(x, y),
            kind,
        });
    }
}

fn spawn_tiles(mut tilemap: ResMut<TileMap>) {
    for x in 0..10 {
        for y in 0..10 {
            tilemap.set_tile(
                x,
                y,
                if rand::random::<bool>() {
                    TileKind::Grass
                } else {
                    TileKind::Stone
                },
            );
        }
    }
}

fn update_tiles(
    mut commands: Commands,
    mut tilemap: ResMut<TileMap>,
    tile_sprites: Res<TileSprites>,
    tiles: Query<Entity, With<TileKind>>,
) {
    if !tilemap.is_changed() {
        return;
    }
    let mut existing = Vec::new();
    for tile in tiles.iter() {
        if tilemap.tiles.contains(&tile) {
            existing.push(tile);
        } else {
            commands.entity(tile).despawn();
        }
    }
    let to_be_added = mem::take(&mut tilemap.to_be_added);
    for tile in to_be_added {
        let tile_ent = commands
            .spawn()
            .insert_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(tile.pos.x as f32, tile.pos.y as f32, 0.0),
                    ..default()
                },
                texture: tile_sprites.sprites.get(&tile.kind).unwrap().clone(),
                sprite: Sprite {
                    custom_size: Some((1.0, 1.0).into()),
                    ..Default::default()
                },
                ..default()
            })
            .insert(tile.kind)
            .insert(Name::new("Tile"))
            .insert(DebugRect {
                color: Color::BLACK,
                size: Vec2::splat(0.9),
                ..default()
            })
            .id();
        tilemap.tiles.push(tile_ent);
    }
}

fn red_if_occupied(
    mut tiles: Query<(&Transform, &mut DebugRect), With<TileKind>>,
    player: Query<&Transform, (With<Player>, Without<TileKind>)>,
) {
    let player_tf = player.single();
    for (tile_tf, mut rect) in tiles.iter_mut() {
        let tile_xy = tile_tf.translation.xy();
        let player_xy = player_tf.translation.xy();
        rect.color = if tile_xy.abs_diff_eq(player_xy, 0.5) {
            Color::RED
        } else {
            Color::BLACK
        }
    }
}
