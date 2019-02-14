//! ## Shader Inputs
//!
//! ### Uniforms
//! - 0: mat4 u_camera
//! - 1: sampler u_texture
//!
//! ### Per-Vertex
//! - 0: vec2 a_pos
//! - 1: vec2 a_uv
//!
//! ### Per-Instance
//! - 2: vec4 i_uvrect
//! - 3: vec4 i_transform0
//! - 4: vec4 i_transform1
//! - 5: float i_layer
//! - 6: int i_imagelayer

use crate::graphics::{
    core::GraphicsCore,
    hal::{self, buffer, pso, Backend, Device},
    B,
};

use failure::{Error, Fail};

#[repr(C)]
#[derive(Copy, Clone)]
struct QuadVertex {
    a_pos: [f32; 2],
    a_uv: [f32; 2],
}

const fn qvert(x: f32, y: f32, u: f32, v: f32) -> QuadVertex {
    QuadVertex {
        a_pos: [x, y],
        a_uv: [u, v],
    }
}

static QUADS: [QuadVertex; 6] = [
    qvert(-0.5, 0.5, 0.0, 1.0),
    qvert(0.5, 0.5, 1.0, 1.0),
    qvert(0.5, -0.5, 1.0, 0.0),
    qvert(-0.5, 0.5, 0.0, 1.0),
    qvert(0.5, -0.5, 1.0, 0.0),
    qvert(-0.5, -0.5, 0.0, 0.0),
];

pub struct QuadInstance {
    pub uv_rect: [f32; 4],
    pub transform: [[f32; 3]; 2],
    pub layer: f32,
    pub image_layer: i32,
}

pub struct SimpleQuadShader {
    pub quad_buffer: <B as Backend>::Buffer,
    pub set_layout: <B as Backend>::DescriptorSetLayout,
    pub desc_set: <B as Backend>::DescriptorSet,
    pub pipeline_layout: <B as Backend>::PipelineLayout,
    pub pipeline: <B as Backend>::GraphicsPipeline,
}

pub fn load(core: &mut GraphicsCore) -> Result<SimpleQuadShader, Error> {
    let quad_buffer = unsafe {
        let buffer_stride = std::mem::size_of::<QuadVertex>() as u64;
        let buffer_len = buffer_stride * QUADS.len() as u64;

        let mut buffer = core
            .device
            .create_buffer(buffer_len, buffer::Usage::VERTEX)?;
        let buffer_req = core.device.get_buffer_requirements(&buffer);

        let upload_type = core
            .mem_properties
            .memory_types
            .iter()
            .enumerate()
            .position(|(id, mem_type)| {
                buffer_req.type_mask & (1 << id) != 0
                    && mem_type
                        .properties
                        .contains(hal::memory::Properties::CPU_VISIBLE)
            })
            .unwrap()
            .into();

        let buffer_mem = core.device.allocate_memory(upload_type, buffer_req.size)?;

        core.device
            .bind_buffer_memory(&buffer_mem, 0, &mut buffer)?;
        let mut vertices = core
            .device
            .acquire_mapping_writer::<QuadVertex>(&buffer_mem, 0..buffer_req.size)?;
        vertices[0..QUADS.len()].copy_from_slice(&QUADS);
        core.device.release_mapping_writer(vertices)?;

        buffer
    };

    let (set_layout, desc_set, pipeline_layout) = unsafe {
        use crate::graphics::hal::pso::DescriptorPool;
        let layout = core.device.create_descriptor_set_layout(
            &[
                pso::DescriptorSetLayoutBinding {
                    binding: 0,
                    ty: pso::DescriptorType::UniformBuffer,
                    count: 1,
                    stage_flags: pso::ShaderStageFlags::VERTEX,
                    immutable_samplers: false,
                },
                pso::DescriptorSetLayoutBinding {
                    binding: 0,
                    ty: pso::DescriptorType::CombinedImageSampler,
                    count: 1,
                    stage_flags: pso::ShaderStageFlags::FRAGMENT,
                    immutable_samplers: false,
                },
            ],
            &[],
        )?;
        let desc_set = core.desc_pool.allocate_set(&layout)?;
        let pipeline_layout = core
            .device
            .create_pipeline_layout(std::iter::once(&layout), &[])?;
        (layout, desc_set, pipeline_layout)
    };

    let pipeline = {
        use gfx_hal::format::Format;

        static VERT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/simple_quad.vert.spirv"));
        static FRAG: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/simple_quad.frag.spirv"));

        let vs_module = unsafe {
            core.device
                .create_shader_module(VERT)
                .map_err(|e| e.context("simple_quad vert shader"))?
        };
        let fs_module = unsafe {
            core.device
                .create_shader_module(&FRAG)
                .map_err(|e| e.context("simple_quad frag shader"))?
        };

        let (vs_entry, fs_entry) = (
            pso::EntryPoint {
                entry: "main",
                module: &vs_module,
                specialization: pso::Specialization::default(),
            },
            pso::EntryPoint {
                entry: "main",
                module: &fs_module,
                specialization: pso::Specialization::default(),
            },
        );

        let pipeline = {
            let shader_entries = pso::GraphicsShaderSet {
                vertex: vs_entry,
                fragment: Some(fs_entry),

                domain: None,
                geometry: None,
                hull: None,
            };

            let subpass = hal::pass::Subpass {
                main_pass: &core.render_pass,
                index: 0,
            };

            let mut pipeline_desc = pso::GraphicsPipelineDesc::new(
                shader_entries,
                hal::Primitive::TriangleList,
                pso::Rasterizer::FILL,
                &pipeline_layout,
                subpass,
            );
            pipeline_desc.blender.targets.push(pso::ColorBlendDesc(
                pso::ColorMask::ALL,
                pso::BlendState::ALPHA,
            ));

            pipeline_desc.vertex_buffers.push(pso::VertexBufferDesc {
                binding: 0,
                stride: std::mem::size_of::<QuadVertex>() as u32,
                rate: 0,
            });
            pipeline_desc.vertex_buffers.push(pso::VertexBufferDesc {
                binding: 1,
                stride: std::mem::size_of::<QuadInstance>() as u32,
                rate: 1,
            });

            // a_pos
            pipeline_desc.attributes.push(pso::AttributeDesc {
                binding: 0,
                location: 0,
                element: pso::Element {
                    format: Format::Rg32Float,
                    offset: 0,
                },
            });
            // a_uv
            pipeline_desc.attributes.push(pso::AttributeDesc {
                binding: 0,
                location: 1,
                element: pso::Element {
                    format: Format::Rg32Float,
                    offset: 8,
                },
            });

            // i_uvrect
            pipeline_desc.attributes.push(pso::AttributeDesc {
                binding: 1,
                location: 2,
                element: pso::Element {
                    format: Format::Rgba32Float,
                    offset: 0,
                },
            });
            // i_transform0
            pipeline_desc.attributes.push(pso::AttributeDesc {
                binding: 1,
                location: 3,
                element: pso::Element {
                    format: Format::Rgba32Float,
                    offset: 16,
                },
            });
            // i_transform1
            pipeline_desc.attributes.push(pso::AttributeDesc {
                binding: 1,
                location: 4,
                element: pso::Element {
                    format: Format::Rgba32Float,
                    offset: 28,
                },
            });
            // i_layer
            pipeline_desc.attributes.push(pso::AttributeDesc {
                binding: 1,
                location: 5,
                element: pso::Element {
                    format: Format::R32Float,
                    offset: 40,
                },
            });
            // i_imagelayer
            pipeline_desc.attributes.push(pso::AttributeDesc {
                binding: 1,
                location: 6,
                element: pso::Element {
                    format: Format::R32Int,
                    offset: 44,
                },
            });

            unsafe { core.device.create_graphics_pipeline(&pipeline_desc, None)? }
        };

        unsafe {
            core.device.destroy_shader_module(vs_module);
            core.device.destroy_shader_module(fs_module);
        }

        pipeline
    };

    Ok(SimpleQuadShader {
        quad_buffer,
        desc_set,
        set_layout,
        pipeline_layout,
        pipeline,
    })
}

impl SimpleQuadShader {
    pub unsafe fn destroy(self, core: &GraphicsCore) {
        core.device.destroy_descriptor_set_layout(self.set_layout);
        core.device.destroy_graphics_pipeline(self.pipeline);
        core.device.destroy_pipeline_layout(self.pipeline_layout);
        core.device.destroy_buffer(self.quad_buffer);
    }
}
