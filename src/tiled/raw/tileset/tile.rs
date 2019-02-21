use crate::tiled::raw::context::ParseContext;
use crate::tiled::raw::TileId;

use failure::Fallible;
use xml::attribute as xa;

#[derive(Clone, Debug)]
pub struct Tile {
    pub id: TileId,
    pub tiletype: Option<String>,
    pub terrain: Option<String>,
    pub probability: Option<f32>,
    pub properties: Option<TileProperties>,
}

impl Tile {
    pub fn parse_tag(context: &mut ParseContext, attrs: &[xa::OwnedAttribute]) -> Fallible<Tile> {
        parse_tag! {
            context; attrs;
            <tile
                id = "id"(TileId)
                ?tiletype = "type"(String)
                ?terrain = "terrain"(String)
                ?probability = "probability"(f32)
            >

                <properties> => TileProperties::parse_tag,

            </tile>
        };

        let properties = properties.first().cloned();

        Ok(Tile {
            id,
            tiletype,
            terrain,
            probability,
            properties,
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct TileProperties {
    pub properties: Vec<TileProperty>,
}

impl TileProperties {
    pub fn parse_tag(
        context: &mut ParseContext,
        attrs: &[xa::OwnedAttribute],
    ) -> Fallible<TileProperties> {
        parse_tag! {
            context; attrs;
            <properties>
                <property> => TileProperty::parse_tag,
            </properties>
        }

        let properties = property;

        Ok(TileProperties { properties })
    }
}

#[derive(Clone, Debug)]
pub struct TileProperty {
    pub name: String,
    pub value: String,
    pub kind: String,
}

impl TileProperty {
    pub fn parse_tag(
        context: &mut ParseContext,
        attrs: &[xa::OwnedAttribute],
    ) -> Fallible<TileProperty> {
        parse_tag! {
            context; attrs;
            <property name="name"(String) value="value"(String) ?kind="type"(String) />
        };

        let kind = kind.unwrap_or_else(|| "string".into());

        Ok(TileProperty { name, value, kind })
    }
}
