use crate::graphics::textures::{SubtextureId, TextureId};

#[derive(Copy, Clone)]
pub struct Sprite {
    pub texture: TextureId,
    pub subtexture: SubtextureId,
}
