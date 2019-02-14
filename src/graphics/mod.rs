use failure::ResultExt;

pub mod camera;
pub mod core;
pub mod shaders;
pub mod systems;
pub mod textures;
pub mod wrappers;
pub mod frame;

pub struct GraphicsState {
    pub core: core::GraphicsCore,
    pub shaders: shaders::Shaders,
    pub camera: camera::Camera,
    pub textures: textures::TextureManager,
    pub frame: frame::CurrentFrame,
    pub is_fullscreen: bool,
}

impl GraphicsState {
    pub fn new() -> Result<Self, failure::Error> {
        let core = core::init_graphics().context("initializing graphics core")?;
        let shaders = shaders::Shaders::new(&core).context("loading shaders")?;
        let camera = camera::Camera::new(&core).context("initializing camera")?;
        let textures = textures::TextureManager::new(&core).context("creating texture manager")?;
        let frame = Default::default();
        let is_fullscreen = false;

        Ok(GraphicsState {
            core,
            shaders,
            camera,
            textures,
            frame,
            is_fullscreen,
        })
    }
}
