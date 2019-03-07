use crate::graphics::shaders::shadow::ShadowInstance;
use crate::{Data, EntityIter};

use glium::index::{NoIndices, PrimitiveType};
use glium::vertex::VertexBuffer;
use glium::{uniform, DrawParameters, Surface};

use failure::ResultExt;

#[derive(Default, conniecs::System)]
#[system_type(Entity)]
#[process(process)]
#[aspect(all(shadow, transform))]
pub struct RenderShadows {
    collect: Vec<ShadowInstance>,
    buffer: Option<VertexBuffer<ShadowInstance>>,
}

fn process(r: &mut RenderShadows, entities: EntityIter, data: &mut Data) {
    for entity in entities {
        let shadow = &data.components.shadow[entity];
        let t = &data.components.transform[entity];

        let size = shadow.scale * (t.altitude.abs() + 1.0).powf(shadow.size_factor);
        r.collect.push(ShadowInstance {
            pos: t.pos.into(),
            size,
            z: t.z_layer,
        });
    }

    if r.collect.is_empty() {
        return;
    }

    let graphics = &mut data.services.graphics;
    let mut buffer = r
        .buffer
        .take()
        .and_then(|buf| {
            if buf.len() < r.collect.len() {
                None
            } else {
                Some(buf)
            }
        })
        .unwrap_or_else(|| {
            VertexBuffer::empty_dynamic(&graphics.core.display, r.collect.len()).unwrap()
        });

    buffer
        .slice_mut(..r.collect.len())
        .unwrap()
        .write(&r.collect);

    let shader = &graphics.shaders.shadow;
    let ibuf = buffer.slice(..r.collect.len()).unwrap();
    let buffers = (&shader.verts, ibuf.per_instance().unwrap());

    let camera = graphics.camera.buffer();

    let frame = graphics.frame.gameplay_frame().unwrap();
    frame
        .draw(
            buffers,
            NoIndices(PrimitiveType::TrianglesList),
            &shader.program,
            &glium::uniform! {
                Camera: camera,
            },
            &DrawParameters {
                depth: glium::Depth {
                    test: glium::DepthTest::IfMoreOrEqual,
                    write: true,
                    ..Default::default()
                },
                blend: glium::Blend::alpha_blending(),
                ..Default::default()
            },
        )
        .with_context(|_| format!("drawing {} shadows", r.collect.len()))
        .unwrap();

    r.buffer = Some(buffer);
    r.collect.clear();
}
