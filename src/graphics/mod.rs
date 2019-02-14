pub mod camera;
pub mod core;
pub mod shaders;
pub mod systems;
pub mod textures;
pub mod wrappers;

pub struct GraphicsState {
    pub core: core::GraphicsCore,
    pub shaders: shaders::Shaders,
    pub camera: camera::Camera,
}

impl GraphicsState {
    pub fn new() -> Result<Self, failure::Error> {
        let core = core::init_graphics()?;
        let shaders = shaders::Shaders::new(&core)?;
        let camera = camera::Camera::new(&core)?;

        Ok(GraphicsState {
            core,
            shaders,
            camera,
        })
    }
}
