use crate::tiled::raw::context::ParseContext;
use crate::tiled::raw::context::ParseOrder;
use crate::tiled::raw::data::Data;
use crate::tiled::raw::properties::Properties;

use failure::Fallible;
use xml::attribute as xa;

pub struct TileLayer {
    pub parse_order: ParseOrder,
    pub id: i32,
    pub name: String,
    pub offsetx: f32,
    pub offsety: f32,
    pub opacity: f32,
    pub visible: bool,
    pub properties: Properties,
    pub data: Data,
}

impl TileLayer {
    pub fn parse_tag(
        context: &mut ParseContext,
        attrs: &[xa::OwnedAttribute],
    ) -> Fallible<TileLayer> {
        let parse_order = context.parseorder();
        parse_tag! {
            context; attrs;
            <layer
                id="id"(i32) name="name"(String)
                ?offsetx="offsetx"(f32) ?offsety="offsety"(f32)
                ?opacity="opacity"(f32) ?visible="visible"(i32)
                >
                <properties> => Properties::parse_tag,
                <data> => Data::parse_tag,
            </layer>
        }

        let offsetx = offsetx.unwrap_or(0.0);
        let offsety = offsety.unwrap_or(0.0);
        let opacity = opacity.unwrap_or(1.0);
        let visible = visible.map(|i| i != 0).unwrap_or(true);
        let properties = properties.pop().unwrap_or_default();
        let data = data.pop().ok_or_else(|| failure::err_msg("<layer> is missing <data> tag"))?;

        Ok(TileLayer {
            parse_order,
            id,
            name,
            offsetx,
            offsety,
            opacity,
            visible,
            properties,
            data,
        })
    }
}
