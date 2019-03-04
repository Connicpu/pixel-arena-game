use crate::tiled::map::data::chunk::Chunk;

use math2d::{Point2f, Point2i, Sizef};
use std::collections::HashMap;

pub mod chunk;

#[derive(Serialize, Deserialize)]
pub struct TileData {
    pub tile_size: Sizef,
    pub chunk_width: i32,
    pub chunk_height: i32,
    pub chunks: HashMap<Point2i, Chunk>,
}

impl TileData {
    pub fn tile_at(&self, pos: Point2f) -> Point2i {
        let x = (pos.x / self.tile_size.width).round() as i32;
        let y = (pos.y / self.tile_size.height).round() as i32;
        (x, y).into()
    }

    pub fn chunk_pos(&self, tile_pos: Point2i) -> Point2i {
        let x = tile_pos.x.div_euclid(self.chunk_width);
        let y = tile_pos.y.div_euclid(self.chunk_height);
        (x, y).into()
    }
}
