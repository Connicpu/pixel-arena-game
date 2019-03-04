use crate::tiled::raw::context::ParseContext;
use crate::tiled::raw::LocalTileId;

use failure::Fallible;
use xml::attribute as xa;

#[derive(Debug)]
pub struct Animation {
    pub frames: Vec<Frame>,
}

impl Animation {
    pub fn parse_tag(
        context: &mut ParseContext,
        attrs: &[xa::OwnedAttribute],
    ) -> Fallible<Animation> {
        parse_tag! {
            context; attrs;
            <animation>
                <frame> => Frame::parse_tag,
            </animation>
        }

        Ok(Animation { frames: frame })
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Frame {
    pub tileid: LocalTileId,
    pub duration: std::time::Duration,
}

impl Frame {
    pub fn parse_tag(context: &mut ParseContext, attrs: &[xa::OwnedAttribute]) -> Fallible<Frame> {
        parse_tag! {
            context; attrs;
            <frame tileid="tileid"(LocalTileId) duration="duration"(f64) />
        }

        let duration = std::time::Duration::from_micros((duration * 1000.0) as u64);

        Ok(Frame { tileid, duration })
    }
}
