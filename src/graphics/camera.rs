use crate::graphics::core::GraphicsCore;

use glium::uniforms::UniformBuffer;
use glium::implement_uniform_block;
use math2d::Vector2f;

use failure::ResultExt;

pub struct Camera {
    buffer: UniformBuffer<CamBuf>,

    pub position: Vector2f,
    pub offset: Vector2f,
    pub rotation: f32,
    pub height: f32,
    pub skew: Vector2f,

    pub near_z: f32,
    pub far_z: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CamBuf {
    u_camera: [[f32; 4]; 4],
}

implement_uniform_block!(CamBuf, u_camera);

impl Camera {
    pub fn new(core: &GraphicsCore) -> Result<Self, failure::Error> {
        let camera = Camera {
            buffer: UniformBuffer::empty_dynamic(&core.display)
                .context("creating camera uniform buffer")?,

            position: Default::default(),
            offset: Default::default(),
            rotation: 0.0,
            height: 8.0,
            skew: Default::default(),

            near_z: 100.0,
            far_z: -100.0,
        };

        Ok(camera)
    }

    pub fn buffer(&self) -> &UniformBuffer<CamBuf> {
        &self.buffer
    }

    pub fn upload(&self, core: &GraphicsCore) {
        let m = self.calc_view_mat();
        let cambuf = self.make_cambuf(&m, core);
        self.buffer.write(&cambuf);
    }

    fn calc_view_mat(&self) -> math2d::Matrix3x2f {
        use math2d::Matrix3x2f;
        let scale = [self.height / 2.0, self.height / 2.0];

        let mat = Matrix3x2f::translation(self.position + self.offset)
            * Matrix3x2f::scaling(scale, (0.0, 0.0))
            * Matrix3x2f::skew(self.skew.x, self.skew.y, (0.0, 0.0))
            * Matrix3x2f::rotation(self.rotation, (0.0, 0.0));

        mat.inverse()
    }

    fn make_cambuf(&self, m: &math2d::Matrix3x2f, core: &GraphicsCore) -> CamBuf {
        let window_size = core.window().get_inner_size().unwrap();
        let p = (window_size.height / window_size.width) as f32;

        // Z-scale
        let zsc = -2.0 / (self.far_z - self.near_z);
        // Z-base
        let zbs = -(self.far_z + self.near_z) / (self.far_z - self.near_z);

        // This has to be in transpose form for some reason?
        let cambuf = CamBuf {
            u_camera: [
                [m.a * p, m.b, 0.0, 0.0],
                [m.c * p, m.d, 0.0, 0.0],
                [0.0 * p, 0.0, zsc, 0.0],
                [m.x * p, m.y, zbs, 1.0],
            ],
        };

        cambuf
    }
}
