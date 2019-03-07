use crate::tiled::map::tiledata::{chunk::Chunk, TileData, CHUNK_SIZE};
use crate::tiled::map::tilesets::Tilesets;
use crate::tiled::raw;

use std::collections::HashMap;

use failure::{err_msg, Fallible};
use math2d::Vector2f;

#[derive(Serialize, Deserialize)]
pub enum Layer {
    Tile(TileLayer),
    Unused,
}

impl Layer {
    pub fn from_raw(raw: &raw::Layer, sets: &Tilesets, tile_size: Vector2f) -> Fallible<Self> {
        match raw {
            raw::Layer::Tile(raw) => TileLayer::from_raw(raw, sets, tile_size).map(Layer::Tile),
            raw::Layer::Object(_) => Err(err_msg("TODO: Object layers")),
            raw::Layer::Image(_) => Err(err_msg("TODO: Image layers")),
            raw::Layer::Group(_) => Err(err_msg("TODO: Object layers")),
        }
    }

    pub fn validate(&self, sets: &Tilesets) -> Fallible<()> {
        match self {
            Layer::Tile(layer) => layer.validate(sets),
            Layer::Unused => unimplemented!(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TileLayer {
    pub flags: LayerFlags,
    pub opacity: f32,
    pub visible: bool,
    pub data: TileData,
}

impl TileLayer {
    pub fn from_raw(raw: &raw::TileLayer, sets: &Tilesets, tile_size: Vector2f) -> Fallible<Self> {
        let rawchunks = match &raw.data {
            raw::Data::Plain(_) => return Err(err_msg("TODO: Chunk up non-infinite maps manually")),
            raw::Data::Chunked(chunks) => chunks, // TODO: Handle a potential future where tiled saves uneven chunks
        };

        let flags = LayerFlags::from_raw(&raw.properties)?;
        let opacity = raw.opacity;
        let visible = raw.visible;

        let mut chunks = HashMap::new();
        for raw in rawchunks.iter() {
            if raw.width != CHUNK_SIZE || raw.height != CHUNK_SIZE {
                return Err(err_msg(format!(
                    "Chunk sizes other than {0}x{0} are currently unsupported. \
                     Are you using a newer version of Tiled? \
                     Please bug me about this and I'll try to fix it!",
                    CHUNK_SIZE
                )));
            }

            let pos = TileData::chunk_pos((raw.x, raw.y).into());
            let data: Vec<_> = raw.data.iter().map(|&gid| sets.get_tile(gid)).collect();
            let chunk = Chunk::new(data.into_boxed_slice());
            chunk.validate(sets)?;
            chunks.insert(pos, chunk);
        }

        let data = TileData { tile_size, chunks };

        Ok(TileLayer {
            flags,
            opacity,
            visible,
            data,
        })
    }

    pub fn validate(&self, sets: &Tilesets) -> Fallible<()> {
        self.data.validate(sets)?;

        Ok(())
    }
}

#[auto_enum::enum_flags(u32)]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub enum LayerFlags {
    NOCOLLIDE,

    NONE = 0,
}

impl LayerFlags {
    pub fn from_raw(props: &raw::Properties) -> Fallible<LayerFlags> {
        let mut flags = LayerFlags::NONE;
        if let Some(raw::Property::String(prop)) = props.properties.get("flags") {
            for flag in prop.split('|') {
                flags |= match flag.trim() {
                    "NOCOLLIDE" => LayerFlags::NOCOLLIDE,

                    "" | "NONE" => LayerFlags::NONE,
                    _ => return Err(err_msg(format!("Unknown layer flag `{}`", flag))),
                };
            }
        }
        Ok(flags)
    }
}
