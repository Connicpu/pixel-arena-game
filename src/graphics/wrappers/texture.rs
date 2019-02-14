use crate::graphics::core::GraphicsCore;

use glium::texture::{MipmapsOption, RawImage2d, SrgbTexture2dArray};

pub struct TextureData {
    texture: SrgbTexture2dArray,
}

impl TextureData {
    pub fn new(
        core: &GraphicsCore,
        data: Vec<RawImage2d<u8>>,
        mipmaps: MipmapsOption,
    ) -> Result<Self, failure::Error> {
        let texture = SrgbTexture2dArray::with_mipmaps(&core.display, data, mipmaps)?;
        Ok(TextureData { texture })
    }

    pub fn texture(&self) -> &SrgbTexture2dArray {
        &self.texture
    }
}
