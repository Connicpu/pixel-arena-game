use crate::graphics::core::GraphicsCore;

use failure::Fallible;

pub struct TileChunkShader {
    pub program: glium::Program,
}

static VERT_SHADER: &str = include_str!("tile_chunk/tile_chunk.vert");
static FRAG_SHADER: &str = include_str!("tile_chunk/tile_chunk.frag");

pub fn load(core: &GraphicsCore) -> Fallible<TileChunkShader> {
    let program = glium::Program::from_source(&core.display, VERT_SHADER, FRAG_SHADER, None)?;

    Ok(TileChunkShader { program })
}
