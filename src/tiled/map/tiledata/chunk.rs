use super::CHUNK_SIZE;
use crate::graphics::core::GraphicsCore;
use crate::graphics::GraphicsState;
use crate::physics::MetaBody;
use crate::tiled::map::tilesets::Tilesets;
use crate::tiled::map::TileId;
use crate::tiled::map::TilesetId;

use std::collections::HashSet;

use failure::Fallible;
use glium::implement_vertex;
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter, SamplerWrapFunction};
use glium::VertexBuffer;
use math2d::Point2f;

#[derive(Serialize, Deserialize)]
pub struct Chunk {
    pub used_tilesets: Box<[TilesetId]>,
    pub data: Box<[TileId]>,

    #[serde(skip)] // Don't serialize this field
    buffers: Option<Box<[VertexBuffer<TileInstance>]>>,
    #[serde(skip)]
    physics_body: Option<wrapped2d::b2::BodyHandle>,
}

impl Chunk {
    pub fn new(data: impl Into<Box<[TileId]>>) -> Self {
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
            data,
            used_tilesets,
            buffers: None,
            physics_body: None,
        }
    }

    pub fn validate(&self, sets: &Tilesets) -> Fallible<()> {
        use failure::err_msg;
        if self.data.len() != (CHUNK_SIZE * CHUNK_SIZE) as usize {
            return Err(err_msg("Chunk contains invalid number of tiles"));
        }
        for &tileset in self.used_tilesets.iter() {
            // Make sure the tileset is valid for this map
            if sets.get(tileset).is_none() {
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

    /// Ensure the buffers are initialized. This is safe to call every frame, it's a simple
    /// is_some() check to instant return.
    pub fn initialize(&mut self, core: &GraphicsCore) -> Fallible<()> {
        if self.buffers.is_some() {
            return Ok(());
        }

        let mut buffers = Vec::with_capacity(self.used_tilesets.len());
        let mut data = Vec::with_capacity(self.data.len());

        for &ts_id in self.used_tilesets.iter() {
            // Collect all of the tiles that should render for this tileset
            for (i, &tile) in self.data.iter().enumerate() {
                if tile.tileset() == ts_id {
                    let x = (i as u16 % CHUNK_SIZE as u16) as u8;
                    let y = (i as u16 / CHUNK_SIZE as u16) as u8;
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

    pub fn create_physics(
        &mut self,
        sets: &Tilesets,
        pos: &Point2f,
        physics: &mut crate::physics::World,
    ) {
        use wrapped2d::b2;

        let mut def = b2::BodyDef::new();
        def.body_type = b2::BodyType::Static;
        def.position = [pos.x, pos.y].into();

        let body = physics.create_body(&def);
        self.physics_body = Some(body);

        self.create_fixtures(sets, &mut physics.body_mut(body));
    }

    fn create_fixtures(&self, sets: &Tilesets, body: &mut MetaBody) {
        for (i, &tid) in self.data.iter().enumerate() {
            if let Some(tile) = sets.get_tile(tid) {
                let x = (i % CHUNK_SIZE as usize) as f32 - 0.5;
                let y = -((i / CHUNK_SIZE as usize) as f32) + 0.5;
                tile.create_collider(&(x, y).into(), body);
            }
        }
    }

    pub fn render(
        &self,
        graphics: &mut GraphicsState,
        sets: &Tilesets,
        position: Point2f,
        layer: f32,
    ) -> Fallible<()> {
        let buffers = self.buffers.as_ref().unwrap();
        let frame = graphics.frame.gameplay_frame().unwrap();
        for (i, &ts_id) in self.used_tilesets.iter().enumerate() {
            use glium::index::{NoIndices, PrimitiveType};
            use glium::{uniform, DrawParameters, Surface};

            let tileset = sets.get(ts_id).unwrap();
            let tile_size = tileset.tile_scale;
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
                    tile_size: [tile_size.x, tile_size.y],
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
}

#[derive(Copy, Clone)]
pub struct TileInstance {
    pub tile_pos: [u8; 2],
    pub tile_id: u16,
}

implement_vertex!(TileInstance, tile_pos, tile_id);
