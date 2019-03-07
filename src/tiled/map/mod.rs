use self::layer::Layer;
use self::tilesets::Tilesets;
use crate::tiled::raw;

use failure::Fallible;

pub mod layer;
pub mod tiledata;
pub mod tilesets;

#[derive(Serialize, Deserialize)]
pub struct Map {
    pub tilesets: Tilesets,
    pub layers: Vec<Layer>,
}

impl Map {
    pub fn from_raw(raw: &raw::Map) -> Fallible<Self> {
        let tile_size = [raw.tilewidth as f32, raw.tileheight as f32].into();

        let tilesets = Tilesets::from_raw(&raw.tilesets, tile_size)?;
        let layers: Fallible<_> = raw
            .layers
            .iter()
            .map(|raw| Layer::from_raw(raw, &tilesets, tile_size))
            .collect();
        let layers = layers?;

        Ok(Map { tilesets, layers })
    }

    pub fn validate(&self) -> Fallible<()> {
        self.tilesets.validate()?;
        for layer in self.layers.iter() {
            layer.validate(&self.tilesets)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TileId(u32);

impl TileId {
    pub fn new(tileset: TilesetId, tile: LocalTileId) -> Self {
        TileId((tileset.0 as u32) << 16 | tile.0 as u32)
    }

    pub fn tileset(self) -> TilesetId {
        TilesetId((self.0 >> 16) as u16)
    }

    pub fn tile(self) -> LocalTileId {
        LocalTileId(self.0 as u16)
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LocalTileId(pub u16);

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TilesetId(pub u16);
