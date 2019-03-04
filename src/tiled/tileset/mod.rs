use crate::graphics::core::GraphicsCore;
use crate::tiled::tileset::tile::Tile;

use failure::{Fallible, ResultExt};
use glium::texture::{ClientFormat, RawImage1d, SrgbTexture2d, Texture1d};

pub mod image;
pub mod tile;

#[derive(Serialize, Deserialize)]
pub struct Tileset {
    pub tiles: Box<[Tile]>,
    pub image: image::Image,

    pub tile_width: u16,
    pub tile_height: u16,
    pub rows: u8,
    pub columns: u8,
    pub margin: u8,
    pub spacing: u8,

    #[serde(skip)]
    tile_rect_buffer: Option<Texture1d>,
}

impl Tileset {
    pub fn initialize(&mut self, core: &GraphicsCore) -> Fallible<()> {
        self.image
            .initialize(core)
            .context("Initializing tileset image")?;

        self.create_rect_buffer(core)
            .context("Initializing tileset rect buffer")?;

        Ok(())
    }

    pub fn tileset_image(&self) -> &SrgbTexture2d {
        self.image
            .texture
            .as_ref()
            .expect("Tileset must be initialized after loading")
    }

    pub fn tile_rect_buffer(&self) -> &Texture1d {
        self.tile_rect_buffer
            .as_ref()
            .expect("Tileset must be initialized after loading")
    }

    fn create_rect_buffer(&mut self, core: &GraphicsCore) -> Fallible<()> {
        let mut buf = Vec::with_capacity(self.tiles.len() * 4);

        let tw = self.tile_width as i32;
        let th = self.tile_height as i32;
        let x0 = self.margin as i32;
        let y0 = self.margin as i32;
        let xs = tw + self.spacing as i32;
        let ys = th + self.spacing as i32;
        
        for ty in 0..self.rows as i32 {
            for tx in 0..self.columns as i32 {
                let x = x0 + tx * xs;
                let y = y0 + ty * ys;
                let irect = math2d::Recti::new(x, y, x + tw, y + th);
                let rect = self.image.rect(irect);
                buf.push(rect.left);
                buf.push(rect.top);
                buf.push(rect.right);
                buf.push(rect.bottom);
            }
        }

        let raw = RawImage1d {
            format: ClientFormat::F32F32F32F32,
            width: (buf.len() / 4) as u32,
            data: buf.into(),
        };

        let tex = Texture1d::new(&core.display, raw)?;
        self.tile_rect_buffer = Some(tex);

        Ok(())
    }
}
