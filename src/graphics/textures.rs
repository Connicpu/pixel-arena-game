use crate::graphics::core::GraphicsCore;
use crate::graphics::wrappers::texture::TextureData;

use glium::texture::{MipmapsOption, RawImage2d};
use index_pool::IndexPool;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TextureId(u32);

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SubtextureId(u32);

#[derive(Default)]
pub struct TextureManager {
    temp_textures: Vec<Option<Texture>>,
    temp_free: IndexPool,

    perm_textures: Vec<Texture>,
}

const TEMP_MASK: u32 = 0x8000_0000;

impl TextureManager {
    pub fn new(core: &GraphicsCore) -> Result<Self, failure::Error> {
        let mut tm = Self::default();

        #[rustfmt::skip]
        let def_tex = vec![
            0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF,
            0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF,
        ];
        let def_tex = RawImage2d::from_raw_rgba(def_tex, (2, 2));
        let def_tex = TextureData::new(core, vec![def_tex], MipmapsOption::NoMipmap)?;
        let def_tex = Texture::new(def_tex, 1, 1, true);
        assert_eq!(tm.insert(def_tex, true), TextureId::default());

        Ok(tm)
    }

    pub fn get(&self, id: TextureId) -> Option<&Texture> {
        if id.0 & TEMP_MASK == 0 {
            self.perm_textures.get(id.0 as usize)
        } else {
            self.temp_textures
                .get((id.0 & !TEMP_MASK) as usize)
                .unwrap_or(&None)
                .as_ref()
        }
    }

    pub fn insert(&mut self, texture: Texture, permanent: bool) -> TextureId {
        if permanent {
            let id = self.perm_textures.len() as u32;
            self.perm_textures.push(texture);
            TextureId(id)
        } else {
            let id = self.temp_free.new_id();
            if id == self.temp_textures.len() {
                self.temp_textures.push(Some(texture));
            } else {
                self.temp_textures[id] = Some(texture);
            }
            TextureId(id as u32 | TEMP_MASK)
        }
    }

    pub fn free(&mut self, id: TextureId) -> bool {
        if id.0 & TEMP_MASK == 0 {
            return false;
        }

        let id = (id.0 & !TEMP_MASK) as usize;

        if !self.temp_free.return_id(id).is_ok() {
            return false;
        }

        self.temp_textures[id] = None;

        true
    }

    pub fn free_all_temp(&mut self) {
        self.temp_textures.clear();
        self.temp_free.clear();
    }
}

pub struct Texture {
    pub data: TextureData,
    pub rows: u32,
    pub cols: u32,
    pub layers: u32,
    pub pixel_art: bool,
}

impl Texture {
    pub fn new(data: TextureData, rows: u32, cols: u32, pixel_art: bool) -> Self {
        let layers = data.texture().array_size();
        Texture {
            data,
            rows,
            cols,
            layers,
            pixel_art,
        }
    }

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
