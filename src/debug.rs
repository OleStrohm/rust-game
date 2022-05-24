#![allow(dead_code)]

use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParamItem;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_asset::RenderAsset;
use bevy::render::render_resource::std430::AsStd430;
use bevy::render::render_resource::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, BufferBindingType, ShaderStages, StorageBuffer,
};
use bevy::render::renderer::{RenderDevice, RenderQueue};
use bevy::sprite::{Material2d, Material2dPipeline, Material2dPlugin, MaterialMesh2dBundle};

use crate::camera_controller::{CameraFollower, CameraMoved, CAMERA_SIZE};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveCircles>()
            .add_plugin(Material2dPlugin::<DebugMaterial>::default())
            .add_startup_system(spawn_debug_overlay)
            .add_system(update_debug_overlay.after(CameraMoved));
    }
}

fn update_debug_overlay(
    mut materials: ResMut<Assets<DebugMaterial>>,
    mut overlay: Query<&mut Handle<DebugMaterial>>,
    circles: Query<(&Transform, &DebugCircle)>,
    rects: Query<(&Transform, &DebugRect)>,
) {
    for mut overlay in overlay.iter_mut() {
        *overlay = materials.add(DebugMaterial {
            active_circles: ActiveCircles(
                circles
                    .iter()
                    .map(|(tf, c)| DebugCircleData {
                        color: c.color.as_linear_rgba_f32().into(),
                        center: tf.translation,
                        radius: c.radius,
                    })
                    .collect(),
            ),
            active_lines: ActiveLines(
                rects
                    .iter()
                    .flat_map(|(tf, r)| {
                        [
                            (r, r.top_left(tf), r.top_right(tf)),
                            (r, r.top_right(tf), r.bottom_right(tf)),
                            (r, r.bottom_right(tf), r.bottom_left(tf)),
                            (r, r.bottom_left(tf), r.top_left(tf)),
                        ]
                    })
                    .map(|(r, start, end)| DebugLineData {
                        color: r.color.as_linear_rgba_f32().into(),
                        start,
                        end,
                    })
                    .collect(),
            ),
        });
    }
}

fn spawn_debug_overlay(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<DebugMaterial>>,
) {
    commands
        .spawn()
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2::new(
                    CAMERA_SIZE * 4.0,
                    CAMERA_SIZE * 2.0,
                ))))
                .into(),
            material: materials.add(DebugMaterial {
                active_circles: ActiveCircles(Vec::new()),
                active_lines: ActiveLines(Vec::new()),
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 990.0)),
            ..Default::default()
        })
        .insert(CameraFollower)
        .insert(Name::new("Debug layer"));
}

#[derive(Component, Clone, Copy)]
pub struct DebugCircle {
    pub color: Color,
    pub radius: f32,
}

impl Default for DebugCircle {
    fn default() -> Self {
        DebugCircle {
            color: Color::RED,
            radius: 0.5,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, AsStd430)]
pub struct DebugCircleData {
    pub color: Vec4,
    pub center: Vec3,
    pub radius: f32,
}

#[derive(Component, Clone, Copy)]
pub struct DebugRect {
    pub color: Color,
    pub rotation: f32,
    pub size: Vec2,
}

impl Default for DebugRect {
    fn default() -> Self {
        DebugRect {
            color: Color::RED,
            rotation: 0.0,
            size: Vec2::splat(1.0),
        }
    }
}

impl DebugRect {
    fn top_left(&self, tf: &Transform) -> Vec2 {
        tf.translation.xy() + Vec2::new(-self.size.x / 2.0, self.size.y / 2.0)
    }

    fn top_right(&self, tf: &Transform) -> Vec2 {
        tf.translation.xy() + Vec2::new(self.size.x / 2.0, self.size.y / 2.0)
    }

    fn bottom_left(&self, tf: &Transform) -> Vec2 {
        tf.translation.xy() + Vec2::new(-self.size.x / 2.0, -self.size.y / 2.0)
    }

    fn bottom_right(&self, tf: &Transform) -> Vec2 {
        tf.translation.xy() + Vec2::new(self.size.x / 2.0, -self.size.y / 2.0)
    }
}

#[derive(Debug, Clone, Copy, Default, AsStd430)]
pub struct DebugLineData {
    pub color: Vec4,
    pub start: Vec2,
    pub end: Vec2,
}

#[derive(Debug, Deref, DerefMut, Clone, Default)]
pub struct ActiveCircles(Vec<DebugCircleData>);

#[derive(Debug, Deref, DerefMut, Clone, Default)]
pub struct ActiveLines(Vec<DebugLineData>);

pub struct GpuDebugMaterial {
    pub bind_group: BindGroup,
}

#[derive(Debug, Clone, Default, TypeUuid)]
#[uuid = "0b1ad73c-8919-48f5-8e19-d05292791f47"]
pub struct DebugMaterial {
    active_circles: ActiveCircles,
    active_lines: ActiveLines,
}

impl RenderAsset for DebugMaterial {
    type ExtractedAsset = Self;
    type PreparedAsset = GpuDebugMaterial;
    type Param = (
        SRes<RenderDevice>,
        SRes<Material2dPipeline<Self>>,
        SRes<RenderQueue>,
    );

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        mut material: Self,
        (render_device, pipeline, queue): &mut SystemParamItem<Self::Param>,
    ) -> Result<
        Self::PreparedAsset,
        bevy::render::render_asset::PrepareAssetError<Self::ExtractedAsset>,
    > {
        //println!("Debug circles: {}", material.active_circles.len());
        //println!("Debug lines: {}", material.active_lines.len());

        let mut circle_storage = StorageBuffer::<DebugCircleData>::default();
        circle_storage.append(&mut material.active_circles);
        circle_storage.write_buffer(render_device, queue);

        let mut line_storage = StorageBuffer::<DebugLineData>::default();
        line_storage.append(&mut material.active_lines);
        line_storage.write_buffer(render_device, queue);

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: circle_storage.binding().unwrap(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: line_storage.binding().unwrap(),
                },
            ],
            label: Some("debug_material_bind_group"),
            layout: &pipeline.material2d_layout,
        });

        Ok(GpuDebugMaterial { bind_group })
    }
}

impl Material2d for DebugMaterial {
    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        asset_server.watch_for_changes().unwrap();
        Some(asset_server.load("debug.wgsl"))
    }

    fn bind_group(asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("debug_material_layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        })
    }
}

pub type DebugMesh2dBundle = MaterialMesh2dBundle<DebugMaterial>;
