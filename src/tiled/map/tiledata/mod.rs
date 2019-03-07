use crate::tiled::map::tiledata::chunk::Chunk;
use crate::tiled::map::tilesets::Tilesets;

use std::collections::HashMap;

use failure::{err_msg, Fallible};
use math2d::{Point2f, Point2i, Vector2f};

pub mod chunk;
mod chunk_serialization;

pub const CHUNK_SIZE: i32 = 16;

type ChunkMap = HashMap<Point2i, Chunk>;

#[derive(Serialize, Deserialize)]
pub struct TileData {
    pub tile_size: Vector2f,
    #[serde(with = "chunk_serialization")]
    pub chunks: ChunkMap,
}

impl TileData {
    pub fn validate(&self, sets: &Tilesets) -> Fallible<()> {
        if self.tile_size.len_squared().abs() <= std::f32::EPSILON {
            return Err(err_msg("tile_size is invalid (zero vector)"));
        }
        for chunk in self.chunks.values() {
            chunk.validate(sets)?;
        }
        Ok(())
    }

    pub fn tile_pos_at(&self, world_pos: Point2f) -> Point2i {
        let x = (world_pos.x / self.tile_size.x).floor() as i32;
        let y = (-world_pos.y / self.tile_size.y).floor() as i32;
        (x, y).into()
    }

    pub fn chunk_pos(tile_pos: Point2i) -> Point2i {
        let x = tile_pos.x.div_euclid(CHUNK_SIZE);
        let y = tile_pos.y.div_euclid(CHUNK_SIZE);
        (x, y).into()
    }
}
