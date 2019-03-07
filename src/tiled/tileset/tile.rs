use crate::tiled::raw;

use failure::{err_msg, Fallible};
use math2d::Vector2f;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Tile {
    pub flags: TileFlags,
    pub colliders: Box<[Collider]>,
}

impl Tile {
    pub fn from_raw(raw: &raw::Tile, tile_size: Vector2f) -> Fallible<Self> {
        let flags = TileFlags::from_raw(&raw.properties)?;

        let colliders: Fallible<Vec<Collider>> = raw
            .objects
            .as_ref()
            .map(|obj| &obj.objects[..])
            .unwrap_or(&[])
            .iter()
            .map(|raw| Collider::from_raw(raw, tile_size))
            .collect();
        let colliders = colliders?.into_boxed_slice();

        Ok(Tile { flags, colliders })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Collider {
    pub shape: Shape,
    pub rotation: f32,
    pub flags: TileFlags,
}

impl Collider {
    pub fn from_raw(raw: &raw::Object, tile_size: Vector2f) -> Fallible<Self> {
        use crate::tiled::raw::Shape as RawShape;
        use std::f32::consts::PI;

        let flags = TileFlags::from_raw(&raw.properties)?;
        let rotation = (raw.rotation * PI / 2.0).rem_euclid(PI * 2.0);

        let shape = {
            let pos = (Vector2f::new(raw.x, raw.y) / tile_size).to_point();
            let size = Vector2f::new(raw.width, raw.height) / tile_size;
            let radius = size * 0.5;

            match &raw.shape {
                RawShape::Rectangle => Shape::Rectangle((pos, pos + size).into()),
                RawShape::Point => Shape::Point(pos),
                RawShape::Ellipse => Shape::Ellipse((pos, radius.x, radius.y).into()),
                RawShape::Polygon(rel_points) => Shape::Polygon(
                    rel_points
                        .iter()
                        .map(|&rel| pos + rel / tile_size)
                        .collect(),
                ),
                RawShape::Polyline(rel_points) => Shape::Polyline(
                    rel_points
                        .iter()
                        .map(|&rel| pos + rel / tile_size)
                        .collect(),
                ),
            }
        };

        Ok(Collider {
            shape,
            rotation,
            flags,
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Shape {
    Rectangle(math2d::Rectf),
    Point(math2d::Point2f),
    Ellipse(math2d::Ellipse),
    Polygon(Vec<math2d::Point2f>),
    Polyline(Vec<math2d::Point2f>),
}

#[auto_enum::enum_flags(u32)]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub enum TileFlags {
    NULL,
    WALL,
    LADDER,
    CLIFF,
    VOID,
    PATH,

    NONE = 0,
}

impl TileFlags {
    pub fn from_raw(props: &raw::Properties) -> Fallible<TileFlags> {
        let mut flags = TileFlags::NONE;
        if let Some(raw::Property::String(prop)) = props.properties.get("flags") {
            for flag in prop.split('|') {
                flags |= match flag.trim() {
                    "NULL" => TileFlags::NULL,
                    "WALL" => TileFlags::WALL,
                    "LADDER" => TileFlags::LADDER,
                    "CLIFF" => TileFlags::CLIFF,
                    "VOID" => TileFlags::VOID,
                    "PATH" => TileFlags::PATH,

                    "" | "NONE" => TileFlags::NONE,
                    _ => return Err(err_msg(format!("Unknown tile flag `{}`", flag))),
                };
            }
        }
        Ok(flags)
    }
}
