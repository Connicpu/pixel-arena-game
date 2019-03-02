use crate::tiled::raw::context::ParseContext;

use failure::Fallible;
use xml::attribute as xa;

#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    pub content: String,
    pub fontfamily: Option<String>,
    pub pixelsize: f32,
    pub wrap: bool,
    pub color: math2d::Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikeout: bool,
    pub kerning: bool,
    pub halign: HAlign,
    pub valign: VAlign,
}

impl Text {
    pub fn parse_tag(context: &mut ParseContext, attrs: &[xa::OwnedAttribute]) -> Fallible<Text> {
        use math2d::Color;
        parse_tag! {
            context; attrs;
            <text ?fontfamily="fontfamily"(String)
                  ?pixelsize="pixelsize"(f32)
                  ?wrap="wrap"(i32)
                  ?color="color"(String)
                  ?bold="bold"(i32)
                  ?italic="italic"(i32)
                  ?underline="underline"(i32)
                  ?strikeout="strikeout"(i32)
                  ?kerning="kerning"(i32)
                  ?halign="halign"(String)
                  ?valign="valign"(String)>
                content,
            </text>
        }

        let pixelsize = pixelsize.unwrap_or(16.0);
        let wrap = wrap.map(|i| i != 0).unwrap_or(false);
        let color = color
            .map(|s| Color::from_str_argb(&s))
            .unwrap_or(Ok(Color::BLACK))?;
        let bold = bold.map(|i| i != 0).unwrap_or(false);
        let italic = italic.map(|i| i != 0).unwrap_or(false);
        let underline = underline.map(|i| i != 0).unwrap_or(false);
        let strikeout = strikeout.map(|i| i != 0).unwrap_or(false);
        let kerning = kerning.map(|i| i != 0).unwrap_or(true);

        let halign = match halign.as_ref().map(|s| s.as_str()).unwrap_or("") {
            "center" => HAlign::Center,
            "right" => HAlign::Right,
            "justify" => HAlign::Justified,
            _ => HAlign::Left,
        };
        let valign = match valign.as_ref().map(|s| s.as_str()).unwrap_or("") {
            "center" => VAlign::Center,
            "bottom" => VAlign::Bottom,
            _ => VAlign::Top,
        };

        Ok(Text {
            content,
            fontfamily,
            pixelsize,
            wrap,
            color,
            bold,
            italic,
            underline,
            strikeout,
            kerning,
            halign,
            valign,
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HAlign {
    Left,
    Center,
    Right,
    Justified,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum VAlign {
    Top,
    Center,
    Bottom,
}
