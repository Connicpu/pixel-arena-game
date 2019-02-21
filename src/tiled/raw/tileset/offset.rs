use crate::tiled::raw::context::ParseContext;

use failure::Fallible;

use xml::attribute as xa;

#[derive(Copy, Clone, Debug)]
pub struct TileOffset {
    pub x: i32,
    pub y: i32,
}

impl TileOffset {
    pub fn parse_tag(
        context: &mut ParseContext,
        attrs: &[xa::OwnedAttribute],
    ) -> Fallible<TileOffset> {
        parse_tag!{
            context; attrs;
            <tileoffset x = "x"(i32) y = "y"(i32)/>
        };

        Ok(TileOffset { x, y })
    }
}
