use crate::graphics::wrappers::texture::TextureData;
use crate::graphics::core::GraphicsCore;

use index_pool::IndexPool;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TextureId(u32);

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SubtextureId(u32);

#[derive(Default)]
pub struct TextureManager {
    temp_textures: Vec<Option<Texture>>,
    temp_free: IndexPool,

    perm_textures: Vec<Texture>,
}

const PERM_MASK: u32 = 0x8000_0000;

impl TextureManager {
    pub fn get(&self, id: TextureId) -> Option<&Texture> {
        if id.0 & PERM_MASK != 0 {
            self.perm_textures.get((id.0 & !PERM_MASK) as usize)
        } else {
            self.temp_textures
                .get(id.0 as usize)
                .unwrap_or(&None)
                .as_ref()
        }
    }

    pub fn insert(&mut self, texture: Texture, permanent: bool) -> TextureId {
        if permanent {
            let id = self.perm_textures.len() as u32 | PERM_MASK;
            self.perm_textures.push(texture);
            TextureId(id)
        } else {
            let id = self.temp_free.new_id();
            if id == self.temp_textures.len() {
                self.temp_textures.push(Some(texture));
            } else {
                self.temp_textures[id] = Some(texture);
            }
            TextureId(id as u32)
        }
    }

    pub unsafe fn free(&mut self, id: TextureId, core: &GraphicsCore) -> bool {
        if id.0 & PERM_MASK != 0 {
            return false;
        }

        let id = id.0 as usize;

        if !self.temp_free.return_id(id).is_ok() {
            return false;
        }

        self.temp_textures[id]
            .take()
            .expect("It should be there...")
            .data
            .destroy(core);

        true
    }

    pub unsafe fn free_all_temp(&mut self, core: &GraphicsCore) {
        for tex in self.temp_textures.drain(..) {
            if let Some(tex) = tex {
                tex.data.destroy(core);
            }
        }
        self.temp_free.clear();
    }

    pub unsafe fn destroy(mut self, core: &GraphicsCore) {
        self.free_all_temp(core);
        for tex in self.perm_textures {
            tex.data.destroy(core);
        }
    }
}

pub struct Texture {
    pub data: TextureData,
    pub rows: u32,
    pub cols: u32,
    pub layers: u32,
}

impl Texture {
    pub fn index(&self, row: u32, col: u32, layer: u32) -> SubtextureId {
        let stride = self.cols;
        let pitch = stride * self.rows;
        SubtextureId(layer * pitch + row * stride + col)
    }

    pub fn coord(&self, id: SubtextureId) -> (u32, u32, u32) {
        let stride = self.cols;
        let pitch = stride * self.rows;
        let i = id.0;

        (i % stride, (i % pitch) / stride, i / pitch)
    }

    pub fn uvrect(&self, row: u32, col: u32) -> math2d::Rectf {
        let csize = 1.0 / self.cols as f32;
        let rsize = 1.0 / self.rows as f32;
        [
            col as f32 * csize,
            row as f32 * rsize,
            (col + 1) as f32 * csize,
            (row + 1) as f32 * rsize,
        ]
        .into()
    }
}


