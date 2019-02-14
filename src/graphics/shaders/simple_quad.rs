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
    pub verts: glium::VertexBuffer<QuadVertex>,
}

static VERT_SHADER: &str = include_str!("simple_quad/simple_quad.vert");
static FRAG_SHADER: &str = include_str!("simple_quad/simple_quad.frag");

pub fn load(core: &GraphicsCore) -> Result<SimpleQuadShader, Error> {
    let program = glium::Program::from_source(&core.display, VERT_SHADER, FRAG_SHADER, None)?;
    let verts = glium::VertexBuffer::immutable(&core.display, &QUADS)?;

    Ok(SimpleQuadShader { program, verts })
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct QuadInstance {
    pub i_uvrect: [f32; 4],
    pub i_transform0: [f32; 3],
    pub i_transform1: [f32; 3],
    pub i_layer: f32,
    pub i_imagelayer: i32,
}

glium::implement_vertex!(
    QuadInstance,
    i_uvrect,
    i_transform0,
    i_transform1,
    i_layer,
    i_imagelayer
);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct QuadVertex {
    a_pos: [f32; 2],
    a_uv: [f32; 2],
}

glium::implement_vertex!(QuadVertex, a_pos, a_uv);

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
