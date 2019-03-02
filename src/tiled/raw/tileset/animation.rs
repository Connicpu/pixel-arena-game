use crate::tiled::raw::context::ParseContext;
use crate::tiled::raw::LocalTileId;

use failure::Fallible;
use xml::attribute as xa;

#[derive(Debug, Serialize, Deserialize)]
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

pub mod anmoptarcserde {
    use crate::tiled::raw::tileset::animation::Animation;
    use std::sync::Arc;

    pub fn serialize<S>(img: &Option<Arc<Animation>>, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match img {
            Some(img) => <Option<&Animation> as serde::Serialize>::serialize(&Some(&**img), ser),
            None => <Option<Animation> as serde::Serialize>::serialize(&None, ser)
        }
    }

    pub fn deserialize<'de, D>(de: D) -> Result<Option<Arc<Animation>>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <Option<Animation> as serde::Deserialize>::deserialize(de).map(|o| o.map(Arc::new))
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
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
