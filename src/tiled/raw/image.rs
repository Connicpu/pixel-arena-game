use crate::tiled::raw::context::ParseContext;
use crate::tiled::raw::context::Source;

use failure::{err_msg, Fallible};
use xml::attribute as xa;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    pub source: Source,
    pub width: i32,
    pub height: i32,
    pub transparent: Option<TransColor>,
}

pub mod imgoptarcserde {
    use crate::tiled::raw::image::Image;
    use std::sync::Arc;

    pub fn serialize<S>(img: &Option<Arc<Image>>, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match img {
            Some(img) => <Option<&Image> as serde::Serialize>::serialize(&Some(&**img), ser),
            None => <Option<Image> as serde::Serialize>::serialize(&None, ser)
        }
    }

    pub fn deserialize<'de, D>(de: D) -> Result<Option<Arc<Image>>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <Option<Image> as serde::Deserialize>::deserialize(de).map(|o| o.map(Arc::new))
    }
}

impl Image {
    pub fn parse_tag(context: &mut ParseContext, attrs: &[xa::OwnedAttribute]) -> Fallible<Image> {
        parse_tag! {
            context; attrs;
            <image
                source = "source"(String)
                width = "width"(i32)
                height = "height"(i32)
                ?transparent = "transparent"(TransColor)
            />
        }

        let source = context.source.relative(&source);

        Ok(Image {
            source,
            width,
            height,
            transparent,
        })
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct TransColor(pub u8, pub u8, pub u8);

impl std::str::FromStr for TransColor {
    type Err = failure::Error;
    fn from_str(s: &str) -> Fallible<TransColor> {
        let s = s.trim().trim_start_matches('#');

        if !s.is_ascii() {
            return Err(err_msg("invalid image.trans color"));
        }

        let (r, g, b) = match s.len() {
            3 => (s[0..1].parse()?, s[1..2].parse()?, s[2..3].parse()?),
            6 => (s[0..2].parse()?, s[2..4].parse()?, s[4..6].parse()?),
            _ => return Err(err_msg("invalid image.trans color")),
        };

        Ok(TransColor(r, g, b))
    }
}
