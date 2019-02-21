use crate::graphics::core::GraphicsCore;

use glium::vertex::VertexBuffer;

pub struct ShadowShader {
    pub program: glium::Program,
    pub verts: VertexBuffer<ShadowVertex>,
}

static VERT_SHADER: &str = include_str!("shadow/shadow.vert");
static FRAG_SHADER: &str = include_str!("shadow/shadow.frag");

pub fn load(core: &GraphicsCore) -> Result<ShadowShader, failure::Error> {
    let program = glium::Program::from_source(&core.display, VERT_SHADER, FRAG_SHADER, None)?;
    let verts = glium::VertexBuffer::immutable(&core.display, &CIRCLE)?;

    Ok(ShadowShader { program, verts })
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShadowInstance {
    pub pos: [f32; 2],
    pub size: f32,
    pub z: f32,
}

glium::implement_vertex!(ShadowInstance, pos, size, z);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShadowVertex {
    a_pos: [f32; 2],
}

glium::implement_vertex!(ShadowVertex, a_pos);

const fn vert(x: f32, y: f32) -> ShadowVertex {
    ShadowVertex { a_pos: [x, y] }
}

#[rustfmt::skip]
static CIRCLE: [ShadowVertex; 36 * 3] = [
    vert(1.0, 0.0),
    vert(0.98480775301221, 0.17364817766693),
    vert(0.0, 0.0),

    vert(0.98480775301221, 0.17364817766693),
    vert(0.93969262078591, 0.34202014332567),
    vert(0.0, 0.0),

    vert(0.93969262078591, 0.34202014332567),
    vert(0.86602540378444, 0.5),
    vert(0.0, 0.0),

    vert(0.86602540378444, 0.5),
    vert(0.76604444311898, 0.64278760968654),
    vert(0.0, 0.0),

    vert(0.76604444311898, 0.64278760968654),
    vert(0.64278760968654, 0.76604444311898),
    vert(0.0, 0.0),

    vert(0.64278760968654, 0.76604444311898),
    vert(0.5, 0.86602540378444),
    vert(0.0, 0.0),

    vert(0.5, 0.86602540378444),
    vert(0.34202014332567, 0.93969262078591),
    vert(0.0, 0.0),

    vert(0.34202014332567, 0.93969262078591),
    vert(0.17364817766693, 0.98480775301221),
    vert(0.0, 0.0),

    vert(0.17364817766693, 0.98480775301221),
    vert(6.1232339957368e-17, 1.0),
    vert(0.0, 0.0),

    vert(6.1232339957368e-17, 1.0),
    vert(-0.17364817766693, 0.98480775301221),
    vert(0.0, 0.0),

    vert(-0.17364817766693, 0.98480775301221),
    vert(-0.34202014332567, 0.93969262078591),
    vert(0.0, 0.0),

    vert(-0.34202014332567, 0.93969262078591),
    vert(-0.5, 0.86602540378444),
    vert(0.0, 0.0),

    vert(-0.5, 0.86602540378444),
    vert(-0.64278760968654, 0.76604444311898),
    vert(0.0, 0.0),

    vert(-0.64278760968654, 0.76604444311898),
    vert(-0.76604444311898, 0.64278760968654),
    vert(0.0, 0.0),

    vert(-0.76604444311898, 0.64278760968654),
    vert(-0.86602540378444, 0.5),
    vert(0.0, 0.0),

    vert(-0.86602540378444, 0.5),
    vert(-0.93969262078591, 0.34202014332567),
    vert(0.0, 0.0),

    vert(-0.93969262078591, 0.34202014332567),
    vert(-0.98480775301221, 0.17364817766693),
    vert(0.0, 0.0),

    vert(-0.98480775301221, 0.17364817766693),
    vert(-1.0, 1.2246467991474e-16),
    vert(0.0, 0.0),

    vert(-1.0, 1.2246467991474e-16),
    vert(-0.98480775301221, -0.17364817766693),
    vert(0.0, 0.0),

    vert(-0.98480775301221, -0.17364817766693),
    vert(-0.93969262078591, -0.34202014332567),
    vert(0.0, 0.0),

    vert(-0.93969262078591, -0.34202014332567),
    vert(-0.86602540378444, -0.5),
    vert(0.0, 0.0),

    vert(-0.86602540378444, -0.5),
    vert(-0.76604444311898, -0.64278760968654),
    vert(0.0, 0.0),

    vert(-0.76604444311898, -0.64278760968654),
    vert(-0.64278760968654, -0.76604444311898),
    vert(0.0, 0.0),

    vert(-0.64278760968654, -0.76604444311898),
    vert(-0.5, -0.86602540378444),
    vert(0.0, 0.0),

    vert(-0.5, -0.86602540378444),
    vert(-0.34202014332567, -0.93969262078591),
    vert(0.0, 0.0),

    vert(-0.34202014332567, -0.93969262078591),
    vert(-0.17364817766693, -0.98480775301221),
    vert(0.0, 0.0),

    vert(-0.17364817766693, -0.98480775301221),
    vert(-1.836970198721e-16, -1.0),
    vert(0.0, 0.0),

    vert(-1.836970198721e-16, -1.0),
    vert(0.17364817766693, -0.98480775301221),
    vert(0.0, 0.0),

    vert(0.17364817766693, -0.98480775301221),
    vert(0.34202014332567, -0.93969262078591),
    vert(0.0, 0.0),

    vert(0.34202014332567, -0.93969262078591),
    vert(0.5, -0.86602540378444),
    vert(0.0, 0.0),

    vert(0.5, -0.86602540378444),
    vert(0.64278760968654, -0.76604444311898),
    vert(0.0, 0.0),

    vert(0.64278760968654, -0.76604444311898),
    vert(0.76604444311898, -0.64278760968654),
    vert(0.0, 0.0),

    vert(0.76604444311898, -0.64278760968654),
    vert(0.86602540378444, -0.5),
    vert(0.0, 0.0),

    vert(0.86602540378444, -0.5),
    vert(0.93969262078591, -0.34202014332567),
    vert(0.0, 0.0),

    vert(0.93969262078591, -0.34202014332567),
    vert(0.98480775301221, -0.17364817766693),
    vert(0.0, 0.0),

    vert(0.98480775301221, -0.17364817766693),
    vert(1.0, 0.0),
    vert(0.0, 0.0),
];