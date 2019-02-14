use crate::graphics::textures::{SubtextureId, TextureId};

#[derive(Copy, Clone, Default)]
pub struct Sprite {
    pub texture: TextureId,
    pub subtexture: SubtextureId,
}
