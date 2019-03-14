use crate::graphics::core::GraphicsCore;
use crate::tiled::map::{LocalTileId, TileId, TilesetId};
use crate::tiled::raw;
use crate::tiled::tileset::{tile::Tile, Tileset};

use std::ops::Range;

use failure::{err_msg, Fallible};
use math2d::Vector2f;

#[derive(Serialize, Deserialize)]
pub struct Tilesets {
    tilesets: Vec<(Range<u32>, Tileset)>,
}

impl Tilesets {
    pub fn from_raw(raw: &[raw::MapTileset], tile_size: Vector2f) -> Fallible<Tilesets> {
        let mut tilesets = Vec::with_capacity(raw.len());
        for raw in raw {
            let tileset = Tileset::from_raw(&raw.data, tile_size)?;
            let fgid = raw.firstgid.0;
            let range = fgid..fgid + tileset.tiles.len() as u32;

            tilesets.push((range, tileset));
        }

        let tilesets = Tilesets { tilesets };
        tilesets.validate()?;
        Ok(tilesets)
    }

    pub fn initialize(&mut self, core: &GraphicsCore) -> Fallible<()> {
        for (_, tileset) in self.tilesets.iter_mut() {
            tileset.initialize(core)?;
        }
        Ok(())
    }

    pub fn validate(&self) -> Fallible<()> {
        for tileset in &self.tilesets {
            tileset.1.validate()?;
        }

        let mut prev = 0..0u32;
        for (range, tileset) in self.tilesets.iter() {
            if range.start < prev.end {
                return Err(err_msg("Tile ranges overlap"));
            }
            if range.len() != tileset.tiles.len() {
                return Err(err_msg("Range is inconsistent with the number of tiles"));
            }
            prev = range.clone();
        }

        Ok(())
    }

    pub fn tile_from_raw(&self, gid: raw::GlobalTileId) -> TileId {
        let gid = gid.0;
        for (i, (range, _)) in self.tilesets.iter().enumerate() {
            if gid < range.start {
                break;
            } else if gid < range.end {
                let tileset = TilesetId((i + 1) as u16);
                let tile = LocalTileId((gid - range.start) as u16);
                return TileId::new(tileset, tile);
            }
        }
        TileId::default()
    }

    pub fn get(&self, id: TilesetId) -> Option<&Tileset> {
        let id = id.0 as usize;
        if id == 0 {
            return None;
        }
        self.tilesets.get(id - 1).map(|(_, tileset)| tileset)
    }

    pub fn get_tile(&self, id: TileId) -> Option<&Tile> {
        self.get(id.tileset()).and_then(|set| set.get(id.tile()))
    }
}
