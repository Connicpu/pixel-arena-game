use crate::graphics::core::GraphicsCore;
use crate::graphics::B;

use gfx_hal::{buffer, Backend, Device};
use math2d::Vector2f;

pub struct Camera {
    cam_buffer: <B as Backend>::Buffer,
    cam_buffer_mem: <B as Backend>::Memory,

    pub position: Vector2f,
    pub rotation: f32,
    pub height: f32,
    pub skew: Vector2f,

    pub near_z: f32,
    pub far_z: f32,
}

type CBR = (<B as Backend>::Buffer, <B as Backend>::Memory);

#[repr(C)]
#[derive(Copy, Clone)]
struct CamBuf {
    data: [[f32; 4]; 4],
}

impl Camera {
    pub fn new(core: &GraphicsCore) -> Result<Self, failure::Error> {
        let (cam_buffer, cam_buffer_mem) = unsafe { Self::create_buffer(core)? };

        let camera = Camera {
            cam_buffer,
            cam_buffer_mem,

            position: Default::default(),
            rotation: 0.0,
            height: 16.0,
            skew: Default::default(),

            near_z: 100.0,
            far_z: -100.0,
        };

        Ok(camera)
    }

    pub fn upload(&self, core: &GraphicsCore) -> Result<(), failure::Error> {
        let m = self.calc_view_mat(core);
        let cambuf = self.make_cambuf(&m);

        unsafe { self.upload_buffer(core, &cambuf) }
    }

    pub fn pso_descriptor(&self) -> gfx_hal::pso::Descriptor<B> {
        gfx_hal::pso::Descriptor::Buffer(&self.cam_buffer, Some(0)..Some(64))
    }

    pub unsafe fn destroy(self, core: &GraphicsCore) {
        core.device.destroy_buffer(self.cam_buffer);
        core.device.free_memory(self.cam_buffer_mem);
    }

    fn calc_view_mat(&self, core: &GraphicsCore) -> math2d::Matrix3x2f {
        use math2d::Matrix3x2f;

        let window_size = core.window.get_inner_size().unwrap();
        let aspect = (window_size.width / window_size.height) as f32;
        let scale = [2.0 * aspect / self.height, 2.0 / self.height];

        let mat = Matrix3x2f::translation(-self.position)
            * Matrix3x2f::scaling(scale, (0.0, 0.0))
            * Matrix3x2f::skew(self.skew.x, self.skew.y, (0.0, 0.0))
            * Matrix3x2f::rotation(self.rotation, (0.0, 0.0));

        mat
    }

    fn make_cambuf(&self, m: &math2d::Matrix3x2f) -> CamBuf {
        // Z-scale
        let zsc = -2.0 / (self.far_z - self.near_z);
        // Z-base
        let zbs = -(self.far_z + self.near_z) / (self.far_z - self.near_z);

        let cambuf = CamBuf {
            data: [
                [m.a, m.c, 0.0, m.x],
                [m.b, m.d, 0.0, m.y],
                [0.0, 0.0, zsc, zbs],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        cambuf
    }

    unsafe fn create_buffer(core: &GraphicsCore) -> Result<CBR, failure::Error> {
        use gfx_hal::memory as m;

        let buffer_stride = std::mem::size_of::<CamBuf>() as u64;
        let buffer_len = buffer_stride * 1;

        let mut buffer = core
            .device
            .create_buffer(buffer_len, buffer::Usage::VERTEX)?;
        let buffer_req = core.device.get_buffer_requirements(&buffer);

        let upload_type = core
            .mem_properties
            .memory_types
            .iter()
            .enumerate()
            .position(|(id, mem_type)| {
                buffer_req.type_mask & (1 << id) != 0
                    && mem_type.properties.contains(m::Properties::CPU_VISIBLE)
            })
            .unwrap()
            .into();

        let buffer_mem = core.device.allocate_memory(upload_type, buffer_req.size)?;

        core.device
            .bind_buffer_memory(&buffer_mem, 0, &mut buffer)?;

        Ok((buffer, buffer_mem))
    }

    unsafe fn upload_buffer(
        &self,
        core: &GraphicsCore,
        data: &CamBuf,
    ) -> Result<(), failure::Error> {
        let mut write = core
            .device
            .acquire_mapping_writer::<CamBuf>(&self.cam_buffer_mem, 0..64)?;

        write[0] = *data;

        core.device.release_mapping_writer(write)?;
        Ok(())
    }
}
