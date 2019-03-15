use crate::Data;

use std::f32::{consts::PI, INFINITY, NEG_INFINITY};

use glium::index::{NoIndices, PrimitiveType};
use glium::{implement_vertex, VertexBuffer};
use glium::{uniform, DrawParameters, Surface};
use math2d::{Point2f, Rectf};
use wrapped2d::b2;

#[derive(Default, conniecs::System)]
#[process = "draw"]
pub struct PhysicsDebugDraw;

#[derive(Copy, Clone)]
pub struct DebugVertex {
    a_pos: [f32; 2],
    a_color: [f32; 4],
}

impl DebugVertex {
    pub fn new(p: &b2::Vec2, c: &b2::Color) -> Self {
        DebugVertex {
            a_pos: [p.x, p.y],
            a_color: [c.r, c.g, c.b, c.a],
        }
    }
    
    pub fn m2d(p: Point2f, c: &b2::Color) -> Self {
        DebugVertex {
            a_pos: [p.x, p.y],
            a_color: [c.r, c.g, c.b, c.a],
        }
    }
}

implement_vertex!(DebugVertex, a_pos, a_color);

fn draw(_: &mut PhysicsDebugDraw, data: &mut Data) {
    let viewport = data.services.graphics.camera.world_viewport();
    let mut lines = Vec::with_capacity(4096);
    let mut triangles = Vec::with_capacity(4096);
    let flags = b2::DRAW_SHAPE;

    data.services.box2d.draw_debug_data(
        &mut DebugCollector::new(viewport, &mut lines, &mut triangles),
        flags,
    );

    let graphics = &mut data.services.graphics;
    let core = &graphics.core;
    let lines = VertexBuffer::immutable(&core.display, &lines).unwrap();
    let triangles = VertexBuffer::immutable(&core.display, &triangles).unwrap();
    let shader = &graphics.shaders.box2d_debug;

    let frame = graphics.frame.gameplay_frame().unwrap();
    frame
        .draw(
            &triangles,
            NoIndices(PrimitiveType::TrianglesList),
            &shader.program,
            &uniform! {
                Camera: graphics.camera.buffer(),
            },
            &DrawParameters {
                blend: glium::Blend::alpha_blending(),
                ..Default::default()
            },
        )
        .unwrap();

    frame
        .draw(
            &lines,
            NoIndices(PrimitiveType::LinesList),
            &shader.program,
            &uniform! {
                Camera: graphics.camera.buffer(),
            },
            &DrawParameters {
                blend: glium::Blend::alpha_blending(),
                line_width: Some(core.window().get_hidpi_factor() as f32),
                ..Default::default()
            },
        )
        .unwrap();
}

struct DebugCollector<'a> {
    lines: &'a mut Vec<DebugVertex>,
    triangles: &'a mut Vec<DebugVertex>,
    viewport: math2d::Rectf,
}

impl<'a> DebugCollector<'a> {
    pub fn new(
        viewport: math2d::Rectf,
        lines: &'a mut Vec<DebugVertex>,
        triangles: &'a mut Vec<DebugVertex>,
    ) -> Self {
        DebugCollector {
            lines,
            triangles,
            viewport,
        }
    }

    fn maybe_visible(&self, p1: &b2::Vec2, p2: &b2::Vec2) -> bool {
        let p1: Point2f = (p1.x, p1.y).into();
        let p2: Point2f = (p2.x, p2.y).into();
        self.viewport.overlaps(&(p1, p2).into())
    }
}

impl b2::Draw for DebugCollector<'_> {
    fn draw_segment(&mut self, p1: &b2::Vec2, p2: &b2::Vec2, color: &b2::Color) {
        if !self.maybe_visible(p1, p2) {
            return;
        }

        self.lines.push(DebugVertex::new(p1, color));
        self.lines.push(DebugVertex::new(p2, color));
    }

    fn draw_polygon(&mut self, verts: &[b2::Vec2], color: &b2::Color) {
        if verts.len() < 2 {
            return;
        }

        // See if it might intersect
        let mut aabb = Rectf::new(INFINITY, INFINITY, NEG_INFINITY, NEG_INFINITY);
        for vert in verts {
            let p = Point2f::new(vert.x, vert.y);
            aabb = aabb.combined_with((p, p));
        }

        if !self.viewport.overlaps(&aabb) {
            return;
        }

        self.draw_segment(&verts[0], &verts[verts.len() - 1], color);
        for pair in verts.windows(2) {
            self.draw_segment(&pair[0], &pair[1], color);
        }
    }

    fn draw_solid_polygon(&mut self, verts: &[b2::Vec2], color: &b2::Color) {
        if verts.len() < 2 {
            return;
        }
        
        // See if it might intersect
        let mut aabb = Rectf::new(INFINITY, INFINITY, NEG_INFINITY, NEG_INFINITY);
        for vert in verts {
            let p = Point2f::new(vert.x, vert.y);
            aabb = aabb.combined_with((p, p));
        }

        if !self.viewport.overlaps(&aabb) {
            return;
        }

        self.draw_polygon(verts, color);

        let mut color = color.clone();
        color.a *= 0.5;

        if verts.len() == 3 {
            self.triangles.push(DebugVertex::new(&verts[0], &color));
            self.triangles.push(DebugVertex::new(&verts[1], &color));
            self.triangles.push(DebugVertex::new(&verts[2], &color));
            return;
        }

        triangulate(verts.iter().map(|&p| Point2f::new(p.x, p.y)), |tri| {
            self.triangles.push(DebugVertex::m2d(tri[0], &color));
            self.triangles.push(DebugVertex::m2d(tri[1], &color));
            self.triangles.push(DebugVertex::m2d(tri[2], &color));
        });
    }

    fn draw_circle(&mut self, center: &b2::Vec2, radius: f32, color: &b2::Color) {
        let aabb = Rectf::from_center_half_extent((center.x, center.y), [radius, radius]);
        if !self.viewport.overlaps(&aabb) {
            return;
        }

        const DIVISIONS: usize = 16;
        let mut vertices = [ZERO; DIVISIONS];
        for i in 0..DIVISIONS {
            let t = i as f32 / DIVISIONS as f32 * 2.0 * PI;
            let x = center.x + t.cos() * radius;
            let y = center.y + t.sin() * radius;
            vertices[i] = [x, y].into();
        }
        self.draw_polygon(&vertices, color);
    }

    fn draw_solid_circle(
        &mut self,
        center: &b2::Vec2,
        radius: f32,
        _axis: &b2::Vec2,
        color: &b2::Color,
    ) {
        let aabb = Rectf::from_center_half_extent((center.x, center.y), [radius, radius]);
        if !self.viewport.overlaps(&aabb) {
            return;
        }

        const DIVISIONS: usize = 16;
        let mut vertices = [ZERO; DIVISIONS];
        for i in 0..DIVISIONS {
            let t = i as f32 / DIVISIONS as f32 * 2.0 * PI;
            let x = center.x + t.cos() * radius;
            let y = center.y + t.sin() * radius;
            vertices[i] = [x, y].into();
        }
        self.draw_solid_polygon(&vertices, color);
    }

    fn draw_transform(&mut self, xf: &b2::Transform) {
        use math2d::Matrix3x2f as M;
        let p = Point2f::new(xf.pos.x, xf.pos.y);
        let rotate = M::rotation(-xf.rot.angle(), p);

        let r = (p + [0.3, 0.0]) * rotate;
        let a = (p + [0.0, 0.5]) * rotate;
        let a1 = (p + [0.1, 0.4]) * rotate;
        let a2 = (p + [-0.1, 0.4]) * rotate;

        self.draw_segment(&b2v(p), &b2v(r), &RIGHT_COLOR);
        self.draw_segment(&b2v(p), &b2v(a), &UP_COLOR);
        self.draw_segment(&b2v(a), &b2v(a1), &UP_COLOR);
        self.draw_segment(&b2v(a), &b2v(a2), &UP_COLOR);
    }
}

const UP_COLOR: b2::Color = b2::Color {
    r: 0.0,
    g: 1.0,
    b: 0.0,
    a: 1.0,
};

const RIGHT_COLOR: b2::Color = b2::Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

const ZERO: b2::Vec2 = b2::Vec2 { x: 0.0, y: 0.0 };
const fn b2v(p: Point2f) -> b2::Vec2 {
    b2::Vec2 { x: p.x, y: p.y }
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
