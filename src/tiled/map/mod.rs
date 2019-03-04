pub mod data;
pub mod tilesets;

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
