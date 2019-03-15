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

        let mut colliders = Vec::new();
        for desc in raw
            .objects
            .as_ref()
            .map(|obj| &obj.objects[..])
            .unwrap_or(&[])
            .iter()
        {
            Collider::from_raw(desc, tile_size, &mut colliders)?;
        }
        let colliders = colliders.into_boxed_slice();

        Ok(Tile { flags, colliders })
    }

    pub fn create_collider(&self, pos: &Point2f, body: &mut MetaBody) {
        use std::f32::consts::PI;

        use approx::ulps_ne;
        use math2d::{Matrix3x2f as M, RectCorner::*};
        use wrapped2d::b2;

        let pos = pos.to_vector();

        for collider in self.colliders.iter() {
            let mat = M::rotation(-collider.rotation, collider.origin) * M::translation(pos);

            let mut def = b2::FixtureDef::new();
            def.is_sensor = collider.flags.is_sensor();
            match &collider.shape {
                Shape::Rectangle(rect) => {
                    let tl = rect.corner(TopLeft) * mat;
                    let tr = rect.corner(TopRight) * mat;
                    let bl = rect.corner(BottomLeft) * mat;
                    let br = rect.corner(BottomRight) * mat;
                    let shape = b2::PolygonShape::new_with(&[
                        [tl.x, tl.y].into(),
                        [tr.x, tr.y].into(),
                        [bl.x, bl.y].into(),
                        [br.x, br.y].into(),
                    ]);
                    body.create_fixture_with(&shape, &mut def, collider.flags);
                }
                Shape::Ellipse(ellipse) => {
                    let half_vec = Vector2f::new(ellipse.radius_x, -ellipse.radius_y);
                    let center = collider.origin + half_vec;
                    let min_rad = ellipse.radius_x.min(ellipse.radius_y);
                    let max_rad = ellipse.radius_x.max(ellipse.radius_y);

                    // Check if this is an ellipse (not a circle)
                    if ulps_ne!(ellipse.radius_x, ellipse.radius_y) {
                        const DIVISIONS: usize = 16;
                        let mut points: [b2::Vec2; DIVISIONS] = [[0.0, 0.0].into(); DIVISIONS];
                        for i in 0..DIVISIONS {
                            let t = (i as f32 / DIVISIONS as f32) * 2.0 * PI;
                            let x = center.x + ellipse.radius_x * t.cos();
                            let y = center.y + ellipse.radius_y * t.sin();
                            let p = mat.transform_point((x, y));
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
                    let center = center * mat;
                    let shape = b2::CircleShape::new_with([center.x, center.y].into(), min_rad);
                    body.create_fixture_with(&shape, &mut def, collider.flags);
                }
                Shape::Triangle(tri) => {
                    let p1 = tri[0] * mat;
                    let p2 = tri[1] * mat;
                    let p3 = tri[2] * mat;

                    let shape = b2::PolygonShape::new_with(&[bpoint(p1), bpoint(p2), bpoint(p3)]);
                    body.create_fixture_with(&shape, &mut def, collider.flags);
                }
                Shape::Chain(points) => {
                    // Skip if an empty polygon somehow makes it in
                    if points.is_empty() {
                        continue;
                    }

                    // Make an array of our b2 points rotated around the dumb origin that tiled uses
                    let points = points
                        .iter()
                        .map(|&p| p * mat)
                        .map(|p| [p.x, p.y].into())
                        .collect::<Vec<_>>();

                    let shape = b2::ChainShape::new_chain(&points);
                    body.create_fixture_with(&shape, &mut def, collider.flags);
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
    pub origin: Point2f,
    pub flags: TileFlags,
}

impl Collider {
    pub fn from_raw(
        raw: &raw::Object,
        tile_size: Vector2f,
        colliders: &mut Vec<Self>,
    ) -> Fallible<()> {
        use crate::tiled::raw::Shape as RawShape;

        let flags = TileFlags::from_raw(&raw.properties)?;
        let rotation = raw.rotation;
        let origin = (Vector2f::new(raw.x, -raw.y) / tile_size).to_point();

        let shape = {
            let size = Vector2f::new(raw.width, -raw.height) / tile_size;
            let radius = size * 0.5;

            match &raw.shape {
                RawShape::Rectangle => Shape::Rectangle((origin, origin + size).into()),
                RawShape::Point => Shape::Point(origin),
                RawShape::Ellipse => {
                    Shape::Ellipse((origin, radius.x.abs(), radius.y.abs()).into())
                }
                RawShape::Polyline(rel_points) => Shape::Chain(
                    rel_points
                        .iter()
                        .map(|&p| Vector2f::new(p.x, -p.y))
                        .map(|p| origin + p / tile_size)
                        .collect(),
                ),
                RawShape::Polygon(rel_points) => {
                    let points = rel_points
                        .iter()
                        .map(|&p| Vector2f::new(p.x, -p.y))
                        .map(|p| origin + p / tile_size);
                    triangulate(points, |tri| {
                        let shape = Shape::Triangle(tri);
                        colliders.push(Collider {
                            shape,
                            rotation,
                            origin,
                            flags,
                        });
                    });
                    return Ok(());
                }
            }
        };

        colliders.push(Collider {
            shape,
            rotation,
            origin,
            flags,
        });
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Shape {
    Rectangle(math2d::Rectf),
    Point(math2d::Point2f),
    Ellipse(math2d::Ellipse),
    Triangle([math2d::Point2f; 3]),
    Chain(Box<[math2d::Point2f]>),
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

fn triangulate(mut points: impl Iterator<Item = Point2f>, mut addface: impl FnMut([Point2f; 3])) {
    use lyon_path::Path;
    use lyon_tessellation::geometry_builder::simple_builder;
    use lyon_tessellation::{FillTessellator, FillVertex, VertexBuffers};

    let path = {
        let mut builder = Path::builder();
        let p0 = match points.next() {
            Some(p0) => p0,
            None => return,
        };
        builder.move_to(epoint(p0));
        for point in points {
            builder.line_to(epoint(point));
        }
        builder.close();
        builder.build()
    };

    let mut buffers: VertexBuffers<FillVertex, _> = VertexBuffers::new();
    {
        let mut vertex_builder = simple_builder(&mut buffers);
        let mut tesselator = FillTessellator::new();

        let res = tesselator.tessellate_path(path.iter(), &Default::default(), &mut vertex_builder);
        if !res.is_ok() {
            return;
        }
    }

    for i_tri in buffers.indices.chunks(3) {
        let v0 = buffers.vertices[i_tri[0] as usize];
        let v1 = buffers.vertices[i_tri[1] as usize];
        let v2 = buffers.vertices[i_tri[2] as usize];
        
        let p0 = Point2f::new(v0.position.x, v0.position.y);
        let p1 = Point2f::new(v1.position.x, v1.position.y);
        let p2 = Point2f::new(v2.position.x, v2.position.y);

        addface([p0, p1, p2]);
    }
}

fn epoint(p: Point2f) -> euclid::Point2D<f32> {
    [p.x, p.y].into()
}

fn bpoint(p: Point2f) -> wrapped2d::b2::Vec2 {
    [p.x, p.y].into()
}
