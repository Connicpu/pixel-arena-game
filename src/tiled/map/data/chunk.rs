use crate::graphics::core::GraphicsCore;
use crate::graphics::GraphicsState;
use crate::tiled::map::tilesets::Tilesets;
use crate::tiled::map::TileId;
use crate::tiled::map::TilesetId;

use std::collections::HashSet;

use failure::Fallible;
use glium::implement_vertex;
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter, SamplerWrapFunction};
use glium::VertexBuffer;
use math2d::{Point2f, Sizef};

#[derive(Serialize, Deserialize)]
pub struct Chunk {
    pub width: u16,
    pub height: u16,
    pub data: Box<[TileId]>,
    pub used_tilesets: Box<[TilesetId]>,

    #[serde(skip)]
    buffers: Option<Box<[VertexBuffer<TileInstance>]>>,
}

impl Chunk {
    pub fn new(width: u16, height: u16, data: impl Into<Box<[TileId]>>) -> Self {
        let data = data.into();

        let used_tilesets = data
            .iter()
            .map(|t| t.tileset())
            .filter(|t| t.0 != 0)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Chunk {
            width,
            height,
            data,
            used_tilesets,
            buffers: None,
        }
    }

    pub fn validate(&self, sets: &Tilesets) -> Fallible<()> {
        use failure::err_msg;
        for &tileset in self.used_tilesets.iter() {
            // Make sure the tileset is valid for this map
            if sets.by_id(tileset).is_none() {
                return Err(err_msg("Chunk references invalid tileset"));
            }
        }
        for &tile in self.data.iter() {
            // The zero tile is always valid
            if tile == TileId::default() {
                continue;
            }

            // Check that this tileset is referenced in the set for this chunk
            if !self.used_tilesets.contains(&tile.tileset()) {
                return Err(err_msg("Chunk contains erroneous tile"));
            }
        }
        Ok(())
    }

    pub fn render(
        &mut self,
        graphics: &mut GraphicsState,
        sets: &Tilesets,
        position: Point2f,
        tile_size: Sizef,
        layer: f32,
    ) -> Fallible<()> {
        // Make sure the buffers have been initialized
        self.init_buffers(&graphics.core)?;

        let buffers = self.buffers.as_ref().unwrap();
        let frame = graphics.frame.gameplay_frame().unwrap();
        for (i, &ts_id) in self.used_tilesets.iter().enumerate() {
            use glium::index::{NoIndices, PrimitiveType};
            use glium::{uniform, DrawParameters, Surface};

            let tileset = sets.by_id(ts_id).unwrap();
            let tex = tileset.tileset_image();
            let rect_buffer = tileset.tile_rect_buffer();
            let camera = graphics.camera.buffer();

            let buffers = (&graphics.core.quad, buffers[i].per_instance().unwrap());
            let shader = &graphics.shaders.tile_chunk;

            frame.draw(
                buffers,
                NoIndices(PrimitiveType::TrianglesList),
                &shader.program,
                &uniform! {
                    Camera: camera,
                    tex: tex.sampled()
                        .wrap_function(SamplerWrapFunction::Clamp)
                        .minify_filter(MinifySamplerFilter::Linear)
                        .magnify_filter(MagnifySamplerFilter::Nearest)
                        .anisotropy(8),
                    rect_buffer: rect_buffer,
                    chunk_pos: [position.x, position.y],
                    tile_size: [tile_size.width, tile_size.height],
                    layer: layer,
                },
                &DrawParameters {
                    depth: glium::Depth {
                        test: glium::DepthTest::IfMoreOrEqual,
                        write: true,
                        ..Default::default()
                    },
                    blend: glium::Blend::alpha_blending(),
                    ..Default::default()
                },
            )?;
        }

        Ok(())
    }

    fn init_buffers(&mut self, core: &GraphicsCore) -> Fallible<()> {
        if self.buffers.is_some() {
            return Ok(());
        }

        let mut buffers = Vec::with_capacity(self.used_tilesets.len());
        let mut data = Vec::with_capacity(self.data.len());

        for &ts_id in self.used_tilesets.iter() {
            // Collect all of the tiles that should render for this tileset
            for (i, &tile) in self.data.iter().enumerate() {
                if tile.tileset() == ts_id {
                    let x = (i as u16 % self.width) as u8;
                    let y = (i as u16 / self.width) as u8;
                    data.push(TileInstance {
                        tile_pos: [x, y],
                        tile_id: tile.tile().0,
                    });
                }
            }

            buffers.push(VertexBuffer::immutable(&core.display, &data)?);
            data.clear();
        }

        self.buffers = Some(buffers.into_boxed_slice());

        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct TileInstance {
    pub tile_pos: [u8; 2],
    pub tile_id: u16,
}

implement_vertex!(TileInstance, tile_pos, tile_id);
