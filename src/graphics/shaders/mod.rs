use crate::graphics::core::GraphicsCore;

use failure::ResultExt;

pub mod simple_quad;
pub mod shadow;

pub struct Shaders {
    pub simple_quad: simple_quad::SimpleQuadShader,
    pub shadow: shadow::ShadowShader,
}

impl Shaders {
    pub fn new(core: &GraphicsCore) -> Result<Self, failure::Error> {
        let simple_quad = simple_quad::load(core).context("loading simple_quad shader")?;
        let shadow = shadow::load(core).context("loading shadow shader")?;

        Ok(Shaders { simple_quad, shadow })
    }
}
