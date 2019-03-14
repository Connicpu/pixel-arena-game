use crate::graphics::core::GraphicsCore;

use failure::Fallible;

pub struct Box2dDebugShader {
    pub program: glium::Program,
}

static VERT_SHADER: &str = include_str!("box2d_debug/box2d_debug.vert");
static FRAG_SHADER: &str = include_str!("box2d_debug/box2d_debug.frag");

pub fn load(core: &GraphicsCore) -> Fallible<Box2dDebugShader> {
    let program = glium::Program::from_source(&core.display, VERT_SHADER, FRAG_SHADER, None)?;

    Ok(Box2dDebugShader { program })
}
