use crate::graphics::core::GraphicsCore;

use failure::{Fallible, ResultExt};
use glium::texture::{RawImage2d, SrgbTexture2d};

#[derive(Serialize, Deserialize)]
pub struct Image {
    /// The image data in webp format
    pub data: Box<[u8]>,
    pub width: u16,
    pub height: u16,

    #[serde(skip)]
    pub(super) texture: Option<SrgbTexture2d>,
}

impl Image {
    pub(super) fn initialize(&mut self, core: &GraphicsCore) -> Fallible<()> {
        let raw_data = self.decode_raw_image().context("Decoding tileset image")?;

        let dims = (self.width as u32, self.height as u32);
        let raw_image = RawImage2d::from_raw_rgba(raw_data, dims);

        let texture = SrgbTexture2d::new(&core.display, raw_image)
            .context("Creating tileset image texture buffer")?;

        self.texture = Some(texture);
        Ok(())
    }

    pub fn rect(&self, rect: math2d::Recti) -> math2d::Rectf {
        let rect = rect.to_f32();
        let w = self.width as f32;
        let h = self.height as f32;
        [rect.left / w, rect.top / h, rect.right / w, rect.bottom / h].into()
    }

    pub fn from_image(img: image::RgbaImage) -> Fallible<Image> {
        let width = img.width() as u16;
        let height = img.height() as u16;

        let data = unsafe {
            use libwebp_sys::WebPEncodeLosslessRGBA as webp_encode;

            let data: &[u8] = &img;
            let width = width as i32;
            let height = height as i32;
            let stride = width * 4;
            let mut out = std::ptr::null_mut();

            let length = webp_encode(data.as_ptr(), width, height, stride, &mut out);
            if length == 0 {
                return Err(failure::err_msg("WebP image encoding failed"));
            }

            let data: Box<[u8]> = std::slice::from_raw_parts(out, length).into();
            libwebp_sys::WebPFree(out as *mut _);
            data
        };

        Ok(Image {
            data,
            width,
            height,

            texture: None,
        })
    }

    fn decode_raw_image(&self) -> Fallible<Vec<u8>> {
        let stride = self.width as usize * 4;
        let raw_size = stride * self.height as usize;

        if raw_size == 0 || self.data.len() == 0 {
            return Err(failure::err_msg("Image data is invalid"));
        }

        let mut data = vec![0; raw_size];
        unsafe {
            use libwebp_sys::WebPDecodeRGBAInto as webp_decode;
            let in_data = self.data.as_ptr();
            let in_size = self.data.len();
            let out_data = data.as_mut_ptr();
            let result = webp_decode(in_data, in_size, out_data, raw_size, stride as i32);
            if result.is_null() {
                return Err(failure::err_msg("WebP image decoding failed"));
            }
        };
        Ok(data)
    }
}
