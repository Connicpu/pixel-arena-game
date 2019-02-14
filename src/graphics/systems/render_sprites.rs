use crate::{DataHelper, EntityIter};
use crate::graphics::textures::TextureId;
use crate::graphics::shaders::simple_quad::QuadInstance;

use std::collections::HashMap;

#[derive(Default, System)]
#[system_type(Entity)]
#[process(process)]
#[aspect(all(sprite, transform))]
pub struct RenderSprites {
    collect: HashMap<TextureId, Vec<QuadInstance>>,
}

fn process(r: &mut RenderSprites, entities: EntityIter, data: &mut DataHelper) {
    for entity in entities {
        
    }

    for v in r.collect.values_mut() {
        v.clear();
    }
}
