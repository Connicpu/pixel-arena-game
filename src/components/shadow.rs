#[derive(Copy, Clone)]
pub struct Shadow {
    pub scale: f32,
    pub size_factor: f32,
}

impl Default for Shadow {
    fn default() -> Self {
        Shadow {
            scale: 1.25,
            size_factor: -1.0,
        }
    }
}
