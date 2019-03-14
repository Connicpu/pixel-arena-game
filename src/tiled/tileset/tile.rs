use crate::physics::MetaBody;
use crate::tiled::raw;

use failure::{err_msg, Fallible};
use math2d::{Point2f, Vector2f};

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

    pub fn create_collider(&self, pos: &Point2f, body: &mut MetaBody) {
        use std::f32::consts::PI;

        use approx::ulps_ne;
        use math2d::{Matrix3x2f as M, RectCorner::*};
        use wrapped2d::b2;

        let pos = pos.to_vector();

        for collider in self.colliders.iter() {
            let mut def = b2::FixtureDef::new();
            def.is_sensor = collider.flags.is_sensor();
            match &collider.shape {
                Shape::Rectangle(mut rect) => {
                    rect.top = -rect.top;
                    rect.bottom = -rect.bottom;
                    let rect = rect.translated_by(pos);
                    let rotate = M::rotation(collider.rotation, rect.corner(TopLeft));
                    let tl = rect.corner(TopLeft) * rotate;
                    let tr = rect.corner(TopRight) * rotate;
                    let bl = rect.corner(BottomLeft) * rotate;
                    let br = rect.corner(BottomRight) * rotate;
                    let shape = b2::PolygonShape::new_with(&[
                        [tl.x, tl.y].into(),
                        [tr.x, tr.y].into(),
                        [bl.x, bl.y].into(),
                        [br.x, br.y].into(),
                    ]);
                    body.create_fixture_with(&shape, &mut def, collider.flags);
                }
                Shape::Ellipse(ellipse) => {
                    let center = Point2f::new(ellipse.center.x, -ellipse.center.y) + pos;
                    let min_rad = ellipse.radius_x.min(ellipse.radius_y);
                    let max_rad = ellipse.radius_x.max(ellipse.radius_y);

                    // Check if this is an ellipse (not a circle)
                    if ulps_ne!(ellipse.radius_x, ellipse.radius_y) {
                        let rotate = M::rotation(
                            collider.rotation,
                            center - [ellipse.radius_x, ellipse.radius_y],
                        );
                        const DIVISIONS: usize = 36;
                        let mut points: [b2::Vec2; DIVISIONS] = [[0.0, 0.0].into(); DIVISIONS];
                        for i in 0..DIVISIONS {
                            let t = (i as f32 / DIVISIONS as f32) * 2.0 * PI;
                            let x = ellipse.radius_x * t.cos();
                            let y = ellipse.radius_y * t.sin();
                            let p = rotate.transform_point((x, y));
                            points[i] = [p.x, p.y].into();
                        }
                        let shape = b2::ChainShape::new_loop(&points);
                        body.create_fixture_with(&shape, &mut def, collider.flags);

                        // Increase the center mass density to be equivalent to the full ellipse.
                        // The difference in area between a circle with radius `r` and an ellipse
                        // with radii (`r`, `q`) where q > r is simply q / r.
                        def.density *= max_rad / min_rad;
                    }

                    // Create the main shape
                    let shape = b2::CircleShape::new_with([center.x, center.y].into(), min_rad);
                    body.create_fixture_with(&shape, &mut def, collider.flags);
                }
                Shape::Polygon(points) | Shape::Polyline(points) => {
                    // Skip if an empty polygon somehow makes it in
                    if points.is_empty() {
                        continue;
                    }

                    // Make an array of our b2 points rotated around the dumb origin that tiled uses
                    let rotate = M::rotation(collider.rotation, points[0] + pos);
                    let points = points
                        .iter()
                        .map(|&p| Point2f::new(p.x, -p.y))
                        .map(|p| p + pos)
                        .map(|p| p * rotate)
                        .map(|p| [p.x, p.y].into())
                        .collect::<Vec<_>>();

                    // Create either the polygon or chain asked for
                    match &collider.shape {
                        Shape::Polygon(_) => {
                            let shape = b2::PolygonShape::new_with(&points);
                            body.create_fixture_with(&shape, &mut def, collider.flags);
                        }
                        Shape::Polyline(_) => {
                            let shape = b2::ChainShape::new_chain(&points);
                            body.create_fixture_with(&shape, &mut def, collider.flags);
                        }
                        _ => unreachable!(),
                    }
                }
                _ => unimplemented!(),
            }
        }
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
                RawShape::Ellipse => {
                    Shape::Ellipse((pos + radius, radius.x.abs(), radius.y.abs()).into())
                }
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
    LEFT,
    RIGHT,
    UP,
    DOWN,

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
                    "LEFT" => TileFlags::LEFT,
                    "RIGHT" => TileFlags::RIGHT,
                    "UP" => TileFlags::UP,
                    "DOWN" => TileFlags::DOWN,

                    "" | "NONE" => TileFlags::NONE,
                    _ => return Err(err_msg(format!("Unknown tile flag `{}`", flag))),
                };
            }
        }
        Ok(flags)
    }

    pub fn is_sensor(self) -> bool {
        self & (TileFlags::LADDER | TileFlags::PATH) != TileFlags::NONE
    }
}
