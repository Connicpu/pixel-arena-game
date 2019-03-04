use crate::tiled::raw::tileset::animation::Animation;
use crate::tiled::raw::image::Image;
use crate::tiled::raw::objects::ObjectGroup;
use crate::tiled::raw::context::ParseContext;
use crate::tiled::raw::properties::Properties;
use crate::tiled::raw::LocalTileId;

use std::sync::Arc;
use failure::Fallible;
use xml::attribute as xa;

#[derive(Debug)]
pub struct Tile {
    pub id: LocalTileId,
    pub tiletype: Option<String>,
    pub terrain: Option<String>,
    pub probability: Option<f32>,
    pub properties: Properties,
    pub image: Option<Arc<Image>>,
    pub objects: Option<Arc<ObjectGroup>>,
    pub animation: Option<Arc<Animation>>,
}

impl Tile {
    pub fn parse_tag(context: &mut ParseContext, attrs: &[xa::OwnedAttribute]) -> Fallible<Tile> {
        parse_tag! {
            context; attrs;
            <tile
                id = "id"(LocalTileId)
                ?tiletype = "type"(String)
                ?terrain = "terrain"(String)
                ?probability = "probability"(f32)
                >

                <properties> => Properties::parse_tag,
                <image> => Image::parse_tag,
                <objectgroup> => ObjectGroup::parse_tag,
                <animation> => Animation::parse_tag,

            </tile>
        };

        let properties = properties.pop().unwrap_or_default();
        let image = image.pop().map(Arc::new);
        let objects = objectgroup.pop().map(Arc::new);
        let animation = animation.pop().map(Arc::new);

        Ok(Tile {
            id,
            tiletype,
            terrain,
            probability,
            properties,
            image,
            objects,
            animation,
        })
    }
}
