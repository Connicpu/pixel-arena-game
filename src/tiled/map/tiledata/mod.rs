use crate::tiled::map::tiledata::chunk::Chunk;
use crate::tiled::map::tilesets::Tilesets;

use std::collections::HashMap;

use failure::Fallible;
use math2d::{Point2f, Point2i};

pub mod chunk;
mod chunk_serialization;

pub const CHUNK_SIZE: i32 = 16;

type ChunkMap = HashMap<Point2i, Chunk>;

#[derive(Serialize, Deserialize)]
pub struct TileData {
    #[serde(with = "chunk_serialization")]
    pub chunks: ChunkMap,
}

impl TileData {
    pub fn validate(&self, sets: &Tilesets) -> Fallible<()> {
        for chunk in self.chunks.values() {
            chunk.validate(sets)?;
        }
        Ok(())
    }

    pub fn tile_pos_at(&self, world_pos: Point2f) -> Point2i {
        let x = (world_pos.x).round() as i32;
        let y = (-world_pos.y).round() as i32;
        (x, y).into()
    }

    pub fn chunk_pos(tile_pos: Point2i) -> Point2i {
        let x = tile_pos.x.div_euclid(CHUNK_SIZE);
        let y = tile_pos.y.div_euclid(CHUNK_SIZE);
        (x, y).into()
    }
}
