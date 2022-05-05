#![allow(dead_code)]

use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParamItem;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_asset::RenderAsset;
use bevy::render::render_resource::std140::{AsStd140, Std140};
use bevy::render::render_resource::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferInitDescriptor, BufferSize,
    BufferUsages, ShaderStages,
};
use bevy::render::renderer::RenderDevice;
use bevy::sprite::{Material2d, Material2dPipeline, MaterialMesh2dBundle};

pub struct GpuDebugMaterial {
    pub buffer: Buffer,
    pub bind_group: BindGroup,
}

#[derive(Clone, Default, AsStd140)]
pub struct DebugMaterialUniform {
    pub color: Vec4,
}

#[derive(Debug, Clone, Copy, TypeUuid)]
#[uuid = "0b1ad73c-8919-48f5-8e19-d05292791f47"]
pub struct DebugMaterial {
    pub color: Color,
    pub radius: f32,
}

impl RenderAsset for DebugMaterial {
    type ExtractedAsset = Self;

    type PreparedAsset = GpuDebugMaterial;

    type Param = (SRes<RenderDevice>, SRes<Material2dPipeline<Self>>);

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        material: Self,
        (render_device, pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<
        Self::PreparedAsset,
        bevy::render::render_asset::PrepareAssetError<Self::ExtractedAsset>,
    > {
        let value = DebugMaterialUniform {
            color: material.color.as_linear_rgba_f32().into(),
        }
        .as_std140();

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("Debug_material_uniform_buffer"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            contents: value.as_bytes(),
        });

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("debug_material_bind_group"),
            layout: &pipeline.material2d_layout,
        });

        Ok(GpuDebugMaterial { buffer, bind_group })
    }
}

impl Material2d for DebugMaterial {
    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("debug.wgsl"))
    }

    fn bind_group(asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("debug_material_layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(
                        DebugMaterialUniform::std140_size_static() as u64
                    ),
                },
                count: None,
            }],
        })
    }
}

pub type DebugMesh2dBundle = MaterialMesh2dBundle<DebugMaterial>;
