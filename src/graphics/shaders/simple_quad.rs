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

use crate::graphics::core::GraphicsCore;

use failure::Error;

pub struct SimpleQuadShader {
    pub program: glium::Program,
}

static VERT_SHADER: &str = include_str!("simple_quad/simple_quad.vert");
static FRAG_SHADER: &str = include_str!("simple_quad/simple_quad.frag");

pub fn load(core: &GraphicsCore) -> Result<SimpleQuadShader, Error> {
    let program = glium::Program::from_source(&core.display, VERT_SHADER, FRAG_SHADER, None)?;

    Ok(SimpleQuadShader { program })
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct QuadInstance {
    pub i_uvrect: [f32; 4],
    pub i_transform0: [f32; 2],
    pub i_transform1: [f32; 2],
    pub i_transform2: [f32; 2],
    pub i_layer: f32,
    pub i_imagelayer: u32,
}

glium::implement_vertex!(
    QuadInstance,
    i_uvrect,
    i_transform0,
    i_transform1,
    i_transform2,
    i_layer,
    i_imagelayer
);
