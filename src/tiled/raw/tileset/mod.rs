use self::offset::TileOffset;
use crate::tiled::raw::context::ParseContext;
use crate::tiled::raw::image::Image;
use crate::tiled::raw::TileId;

use std::sync::Arc;

use failure::{err_msg, Fallible};

use xml::attribute as xa;

pub mod offset;
pub mod tile;

#[derive(Clone, Debug)]
pub struct MapTileset {
    pub firstgid: TileId,
    pub data: Arc<Tileset>,
}

#[derive(Debug)]
pub struct Tileset {
    pub name: String,
    pub tilewidth: i32,
    pub tileheight: i32,
    pub tilecount: i32,
    pub columns: i32,
    pub spacing: i32,
    pub margin: i32,

    pub offset: Option<TileOffset>,
    pub image: Image,
    pub tiles: Vec<tile::Tile>,
}

impl Tileset {
    pub fn parse_map_tag(
        context: &mut ParseContext,
        attrs: &[xa::OwnedAttribute],
    ) -> Fallible<MapTileset> {
        parse_tag! {
            context; attrs;
            <tileset firstgid = "firstgid"(TileId) ?source = "source"(String)>
        }

        let data = match source {
            Some(source) => Self::parse_file(context, &source)?,
            None => Arc::new(Self::parse_tag(context, attrs)?),
        };

        Ok(MapTileset { firstgid, data })
    }

    pub fn parse_file(context: &mut ParseContext, source: &str) -> Fallible<Arc<Tileset>> {
        let source = context.source.relative(source);
        if let Some(set) = context.tilesets.get(&source) {
            return Ok(set.clone());
        }

        let data = source.read_all()?;
        context.subcontext(&data, source, "tileset", |context, attrs| {
            let set = Arc::new(Tileset::parse_tag(context, attrs)?);
            context.tilesets.insert(context.source.clone(), set.clone());
            return Ok(set);
        })
    }

    pub fn parse_tag(
        context: &mut ParseContext,
        attrs: &[xa::OwnedAttribute],
    ) -> Fallible<Tileset> {
        parse_tag! {
            context; attrs;
            <tileset
                name = "name"(String)
                tilewidth = "tilewidth"(i32)
                tileheight = "tileheight"(i32)
                tilecount = "tilecount"(i32)
                columns = "columns"(i32)
                ?spacing = "spacing"(i32)
                ?margin = "margin"(i32)
            >
                <tileoffset> => TileOffset::parse_tag,
                <image> => Image::parse_tag,
                <tile> => tile::Tile::parse_tag,
            </tileset>
        }

        let spacing = spacing.unwrap_or(0);
        let margin = margin.unwrap_or(0);

        let offset = tileoffset.first().cloned();
        let image = image
            .first()
            .cloned()
            .ok_or_else(|| err_msg("<image> must be specified in a tileset"))?;
        let tiles = tile;

        Ok(Tileset {
            name,
            tilewidth,
            tileheight,
            tilecount,
            columns,
            spacing,
            margin,
            offset,
            image,
            tiles,
        })
    }
}
