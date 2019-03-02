use crate::tiled::raw::context::ParseContext;
use crate::tiled::raw::context::ParseOrder;
use crate::tiled::raw::layer::image::ImageLayer;
use crate::tiled::raw::layer::tile::TileLayer;
use crate::tiled::raw::objects::ObjectGroup;

use failure::Fallible;
use xml::attribute as xa;

pub mod image;
pub mod tile;

pub enum Layer {
    Tile(TileLayer),
    Object(ObjectGroup),
    Image(ImageLayer),
}

impl Layer {
    pub fn parseorder(&self) -> ParseOrder {
        match self {
            Layer::Tile(layer) => layer.parse_order,
            Layer::Object(layer) => layer.parse_order,
            Layer::Image(layer) => layer.parse_order,
        }
    }

    pub fn combine(layers: &mut [Vec<Layer>]) -> Vec<Layer> {
        use std::mem::replace;
        let mut result = replace(&mut layers[0], Default::default());
        for layer in layers.iter_mut().skip(1) {
            result.extend(layer.drain(..));
        }
        result.sort_by_key(|layer| layer.parseorder());
        result
    }

    pub fn parse_tile(context: &mut ParseContext, attrs: &[xa::OwnedAttribute]) -> Fallible<Layer> {
        TileLayer::parse_tag(context, attrs).map(Layer::Tile)
    }

    pub fn parse_obj(context: &mut ParseContext, attrs: &[xa::OwnedAttribute]) -> Fallible<Layer> {
        ObjectGroup::parse_tag(context, attrs).map(Layer::Object)
    }

    pub fn parse_img(context: &mut ParseContext, attrs: &[xa::OwnedAttribute]) -> Fallible<Layer> {
        ImageLayer::parse_tag(context, attrs).map(Layer::Image)
    }
}
