use crate::graphics::core::GraphicsCore;

pub mod simple_quad;

pub struct Shaders {
    pub simple_quad: simple_quad::SimpleQuadShader,
}

impl Shaders {
    pub fn new(core: &GraphicsCore) -> Result<Self, failure::Error> {
        let simple_quad = simple_quad::load(core)?;

        Ok(Shaders { simple_quad })
    }
}
