use crate::graphics::core::GraphicsCore;

use failure::ResultExt;

pub mod box2d_debug;
pub mod shadow;
pub mod simple_quad;
pub mod tile_chunk;

pub struct Shaders {
    pub simple_quad: simple_quad::SimpleQuadShader,
    pub shadow: shadow::ShadowShader,
    pub tile_chunk: tile_chunk::TileChunkShader,
    pub box2d_debug: box2d_debug::Box2dDebugShader,
}

impl Shaders {
    pub fn new(core: &GraphicsCore) -> Result<Self, failure::Error> {
        let simple_quad = simple_quad::load(core).context("loading simple_quad shader")?;
        let shadow = shadow::load(core).context("loading shadow shader")?;
        let tile_chunk = tile_chunk::load(core).context("loading tile_chunk shader")?;
        let box2d_debug = box2d_debug::load(core).context("loading box2d_debug shader")?;

        Ok(Shaders {
            simple_quad,
            shadow,
            tile_chunk,
            box2d_debug,
        })
    }
}
