use crate::graphics::core::GraphicsCore;
use crate::tiled::raw;
use crate::tiled::tileset::tile::Tile;

use failure::{err_msg, Fallible, ResultExt};
use glium::texture::{ClientFormat, RawImage1d, SrgbTexture2d, Texture1d};
use math2d::Vector2f;

pub mod image;
pub mod tile;

#[derive(Serialize, Deserialize)]
pub struct Tileset {
    pub tile_scale: Vector2f,
    pub tile_width: u16,
    pub tile_height: u16,
    pub rows: u8,
    pub columns: u8,
    pub margin: u8,
    pub spacing: u8,

    pub image: image::Image,
    pub tiles: Box<[Tile]>,

    #[serde(skip)]
    tile_rect_buffer: Option<Texture1d>,
}

impl Tileset {
    pub fn from_raw(raw: &raw::Tileset, tile_size: Vector2f) -> Fallible<Tileset> {
        let tile_width = raw.tilewidth as u16;
        let tile_height = raw.tileheight as u16;
        let columns = raw.columns as u8;
        let rows = ((raw.tilecount + raw.columns - 1) / raw.columns) as u8;
        let margin = raw.margin as u8;
        let spacing = raw.spacing as u8;
        let local_tile_size: Vector2f = [tile_width as f32, tile_height as f32].into();
        let tile_scale = local_tile_size / tile_size;

        let image = image::Image::from_raw(raw.image.as_ref()).context("Loading tileset image")?;

        let mut tiles = vec![Tile::default(); raw.tilecount as usize].into_boxed_slice();
        for rawtile in &raw.tiles {
            if let Some(tile) = tiles.get_mut(rawtile.id.0 as usize) {
                *tile = Tile::from_raw(rawtile, tile_size)?;
            } else {
                return Err(err_msg("Invalid tile description. Id out of range."));
            }
        }

        let tileset = Tileset {
            tile_scale,
            tile_width,
            tile_height,
            columns,
            rows,
            margin,
            spacing,
            image,
            tiles,

            tile_rect_buffer: None,
        };

        tileset
            .validate()
            .context("Creating tileset from tiled xml file")?;

        Ok(tileset)
    }

    pub fn validate(&self) -> Fallible<()> {
        let cols = self.columns as usize;
        let rows = self.rows as usize;
        let margin = self.margin as usize;
        let twidth = self.tile_width as usize + self.spacing as usize;
        let theight = self.tile_height as usize + self.spacing as usize;

        let max_count = rows * cols;
        let min_count = max_count - cols + 1;
        if self.tiles.len() < min_count || self.tiles.len() > max_count {
            return Err(err_msg(
                "Tile count does not match number of rows and columns",
            ));
        }

        let used_width = cols * twidth + margin - 1;
        let used_height = rows * theight + margin - 1;
        if used_width > self.image.width as usize || used_height > self.image.height as usize {
            return Err(err_msg("Image is too small for the tile layout specified"));
        }

        Ok(())
    }

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
