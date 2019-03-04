use crate::tiled::map::LocalTileId;
use crate::tiled::map::TileId;
use crate::tiled::map::TilesetId;
use crate::tiled::tileset::Tileset;

use std::ops::Range;

#[derive(Serialize, Deserialize)]
pub struct Tilesets {
    tilesets: Vec<Tileset>,
    ranges: Vec<Range<u32>>,
}

impl Tilesets {
    pub fn get_tile(&self, gid: u32) -> TileId {
        for (i, range) in self.ranges.iter().enumerate() {
            if range.contains(&gid) {
                let tileset = TilesetId(i as u16 + 1);
                let tile = LocalTileId((gid - range.start) as u16);
                return TileId::new(tileset, tile);
            }
            if gid < range.start {
                break;
            }
        }
        TileId::default()
    }

    pub fn by_id(&self, id: TilesetId) -> Option<&Tileset> {
        let id = id.0 as usize;
        if id == 0 || id > self.tilesets.len() {
            return None;
        }
        None
    }
}
