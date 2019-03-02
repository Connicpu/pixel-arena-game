use crate::tiled::raw::context::ParseContext;

use failure::Fallible;

use xml::attribute as xa;

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct TileOffset {
    pub x: f32,
    pub y: f32,
}

impl TileOffset {
    pub fn parse_tag(
        context: &mut ParseContext,
        attrs: &[xa::OwnedAttribute],
    ) -> Fallible<TileOffset> {
        parse_tag!{
            context; attrs;
            <tileoffset x = "x"(f32) y = "y"(f32)/>
        };

        Ok(TileOffset { x, y })
    }
}
