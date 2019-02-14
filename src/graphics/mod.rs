use std::mem::ManuallyDrop;

#[cfg(feature = "dx12")]
pub use gfx_backend_dx12 as backend;
#[cfg(feature = "metal")]
pub use gfx_backend_metal as backend;
#[cfg(feature = "vulkan")]
pub use gfx_backend_vulkan as backend;

pub use self::backend::Backend as B;
pub use gfx_hal as hal;

pub mod camera;
pub mod core;
pub mod shaders;
pub mod systems;
pub mod textures;
pub mod wrappers;

pub struct GraphicsState {
    pub core: ManuallyDrop<core::GraphicsCore>,
    pub shaders: ManuallyDrop<shaders::Shaders>,
    pub camera: ManuallyDrop<camera::Camera>,
}

impl GraphicsState {
    pub fn new() -> Result<Self, failure::Error> {
        let mut core = core::init_graphics()?;
        let shaders = shaders::Shaders::new(&mut core)?;
        let camera = camera::Camera::new(&mut core)?;

        let core = ManuallyDrop::new(core);
        let shaders = ManuallyDrop::new(shaders);
        let camera = ManuallyDrop::new(camera);

        Ok(GraphicsState {
            core,
            camera,
            shaders,
        })
    }
}

impl Drop for GraphicsState {
    fn drop(&mut self) {
        unsafe {
            let core = ManuallyDrop::take(&mut self.core);
            let shaders = ManuallyDrop::take(&mut self.shaders);
            let camera = ManuallyDrop::take(&mut self.camera);

            camera.destroy(&core);
            shaders.destroy(&core);
            core.destroy();
        }
    }
}
