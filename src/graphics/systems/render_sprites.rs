use crate::graphics::shaders::simple_quad::QuadInstance;
use crate::graphics::textures::{SubtextureId, TextureId};
use crate::{Data, EntityIter};

use std::collections::HashMap;

use glium::index::{NoIndices, PrimitiveType};
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter, SamplerWrapFunction};
use glium::vertex::VertexBuffer;
use glium::{DrawParameters, Surface};

use failure::ResultExt;

#[derive(Default, System)]
#[system_type(Entity)]
#[process(process)]
#[aspect(all(sprite, transform))]
pub struct RenderSprites {
    collect: HashMap<TextureId, Vec<QuadInstance>>,
}

fn process(r: &mut RenderSprites, entities: EntityIter, data: &mut Data) {
    let graphics = &mut data.services.graphics;
    let def_tex = graphics.textures.get(Default::default()).unwrap();
    let def_tid = TextureId::default();
    let def_sub = SubtextureId::default();

    let mut max_instances = 0;
    let mut num_draws = 0;
    let dt = data.services.time.delta;

    for entity in entities {
        let transform = &mut data.components.transform[entity];
        let sprite = &data.components.sprite[entity];

        // TEST CODE
        if data.services.jump {
            transform.altitude = (transform.altitude + dt).max(0.0);
        } else {
            transform.altitude = (transform.altitude - dt).max(0.0);
        }
        // TEST CODE

        let tid = sprite.texture;

        let (tex, tid, sub) = graphics
            .textures
            .get(tid)
            .map(|t| (t, tid, sprite.subtexture))
            .unwrap_or((&def_tex, def_tid, def_sub));

        let (row, col, layer) = tex.coord(sub);
        let rect = tex.uvrect(row, col);
        let mat = transform.matrix();

        let instance = QuadInstance {
            i_uvrect: [rect.left, rect.top, rect.right, rect.bottom],
            i_transform0: [mat.a, mat.b],
            i_transform1: [mat.c, mat.d],
            i_transform2: [mat.x, mat.y],
            i_layer: transform.z_layer + transform.altitude,
            i_imagelayer: layer,
        };

        let entry = r.collect.entry(tid).or_default();
        entry.push(instance);
        max_instances = std::cmp::max(max_instances, entry.len());
        if entry.len() == 1 {
            num_draws += 1;
        }
    }

    let num_buffers = {
        use std::cmp::{max, min};
        min(num_draws, max(2, min(8, num_draws / 4 + 1)))
    };
    let make_buf = || {
        VertexBuffer::<QuadInstance>::empty_persistent(&graphics.core.display, max_instances)
            .unwrap()
    };
    let mut instance_buffers = std::iter::repeat_with(make_buf)
        .take(num_buffers)
        .collect::<Vec<_>>();
    let mut cur_buf = 0;

    for (&tid, list) in &r.collect {
        if list.is_empty() {
            continue;
        }

        let shader = &graphics.shaders.simple_quad;
        let tex = graphics.textures.get(tid).unwrap();
        let mag_filter = if tex.pixel_art {
            MagnifySamplerFilter::Nearest
        } else {
            MagnifySamplerFilter::Linear
        };

        let instance_buffer = &mut instance_buffers[cur_buf];
        instance_buffer.slice_mut(..list.len()).unwrap().write(list);

        let ibuf = instance_buffer.slice(..list.len()).unwrap();
        let buffers = (&graphics.core.quad, ibuf.per_instance().unwrap());

        let camera = graphics.camera.buffer();

        let frame = graphics.frame.gameplay_frame().unwrap();
        frame
            .draw(
                buffers,
                NoIndices(PrimitiveType::TrianglesList),
                &shader.program,
                &glium::uniform! {
                    Camera: camera,
                    tex: tex.data.texture().sampled()
                        .wrap_function(SamplerWrapFunction::Clamp)
                        .minify_filter(MinifySamplerFilter::Linear)
                        .magnify_filter(mag_filter)
                        .anisotropy(8)
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
            .with_context(|_| format!("drawing {} sprites with {:?}", list.len(), tid))
            .unwrap();

        cur_buf = (cur_buf + 1) % num_buffers;
    }

    drop(instance_buffers);

    for v in r.collect.values_mut() {
        v.clear();
    }
}
