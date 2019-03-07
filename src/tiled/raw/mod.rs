pub use self::{
    context::{ParseContext, ParseOrder, ParseResult, },
    data::{Chunk, Data},
    image::Image,
    layer::{group::GroupLayer, image::ImageLayer, tile::TileLayer, Layer},
    map::Map,
    objects::{shape::Shape, text::Text, Object, ObjectGroup},
    properties::{Properties, Property},
    tileset::{
        animation::{Animation, Frame},
        offset::TileOffset,
        tile::Tile,
        MapTileset, Tileset,
    },
};

#[macro_use]
pub mod helpers;

pub mod context;
pub mod data;
pub mod image;
pub mod layer;
pub mod map;
pub mod objects;
pub mod properties;
pub mod tileset;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocalTileId(pub u32);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalTileId(pub u32);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DrawOrder {
    Index,
    TopDown,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RenderOrder {
    RightDown,
    RightUp,
    LeftDown,
    LeftUp,
}

impl std::str::FromStr for LocalTileId {
    type Err = <u32 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(LocalTileId)
    }
}

impl std::str::FromStr for GlobalTileId {
    type Err = <u32 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(GlobalTileId)
    }
}
