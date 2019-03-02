use crate::tiled::raw::context::ParseContext;
use crate::tiled::raw::layer::Layer;
use crate::tiled::raw::tileset::MapTileset;
use crate::tiled::raw::RenderOrder;

use failure::Fallible;
use xml::attribute as xa;

pub struct Map {
    pub version: String,
    pub tiledversion: String,
    pub orientation: Orientation,
    pub renderorder: RenderOrder,
    pub width: i32,
    pub height: i32,
    pub tilewidth: i32,
    pub tileheight: i32,
    pub infinite: bool,
    pub hexsidelength: Option<f32>,
    pub staggeraxis: Option<Axis>,
    pub staggerindex: Option<i32>,
    pub backgroundcolor: Option<math2d::Color>,

    pub tilesets: Vec<MapTileset>,
    pub layers: Vec<Layer>,
}

impl Map {
    pub fn parse_tag(context: &mut ParseContext, attrs: &[xa::OwnedAttribute]) -> Fallible<Map> {
        use math2d::Color;

        parse_tag! {
            context; attrs;
            <map
                version="version"(String) tiledversion="tiledversion"(String)
                orientation="orientation"(String) renderorder="renderorder"(String)
                width="width"(i32) height="height"(i32)
                tilewidth="tilewidth"(i32) tileheight="tileheight"(i32)
                ?infinite="infinite"(i32)
                ?hexsidelength="hexsidelength"(f32)
                ?staggeraxis="staggeraxis"(String)
                ?staggerindex="staggerindex"(i32)
                ?backgroundcolor="backgroundcolor"(String)>

                <tileset> => MapTileset::parse_tag,
                <layer> => Layer::parse_tile,
                <objectgroup> => Layer::parse_obj,
                <imagelayer> => Layer::parse_img,
            </map>
        }

        let orientation = match orientation.as_str() {
            "orthogonal" => Orientation::Orthogonal,
            "isometric" => Orientation::Isometric,
            "staggered" => Orientation::Staggered,
            "hexagonal" => Orientation::Hexagonal,
            _ => {
                return Err(failure::err_msg(format!(
                    "Unknown orientation '{}'",
                    orientation
                )));
            }
        };
        let renderorder = match renderorder.as_str() {
            "right-down" => RenderOrder::RightDown,
            "right-up" => RenderOrder::RightUp,
            "left-down" => RenderOrder::LeftDown,
            "left-up" => RenderOrder::LeftUp,
            _ => {
                return Err(failure::err_msg(format!(
                    "Unknown renderorder '{}'",
                    renderorder
                )));
            }
        };
        let infinite = infinite.map(|i| i != 0).unwrap_or(false);
        let staggeraxis = match staggeraxis.as_ref().map(|s| s.as_str()) {
            None => None,
            Some("x") => Some(Axis::X),
            Some("y") => Some(Axis::Y),
            Some(a) => return Err(failure::err_msg(format!("Unknown staggeraxis '{}'", a))),
        };
        let backgroundcolor = backgroundcolor
            .map(|s| Color::from_str_argb(&s))
            .transpose()?;
        let tilesets = tileset;
        let layers = Layer::combine(&mut [layer, objectgroup, imagelayer]);

        Ok(Map {
            version,
            tiledversion,
            orientation,
            renderorder,
            width,
            height,
            tilewidth,
            tileheight,
            infinite,
            hexsidelength,
            staggeraxis,
            staggerindex,
            backgroundcolor,
            tilesets,
            layers,
        })
    }
}

pub enum Orientation {
    Orthogonal,
    Isometric,
    Staggered,
    Hexagonal,
}

pub enum Axis {
    X,
    Y,
}
