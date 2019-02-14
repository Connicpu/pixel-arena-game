use crate::graphics::core::GraphicsCore;

use glium::uniforms::UniformBuffer;
use math2d::Vector2f;

pub struct Camera {
    buffer: UniformBuffer<CamBuf>,

    pub position: Vector2f,
    pub rotation: f32,
    pub height: f32,
    pub skew: Vector2f,

    pub near_z: f32,
    pub far_z: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CamBuf {
    data: [[f32; 4]; 4],
}

impl Camera {
    pub fn new(core: &GraphicsCore) -> Result<Self, failure::Error> {
        let camera = Camera {
            buffer: UniformBuffer::empty_dynamic(&core.display)?,

            position: Default::default(),
            rotation: 0.0,
            height: 16.0,
            skew: Default::default(),

            near_z: 100.0,
            far_z: -100.0,
        };

        Ok(camera)
    }

    pub fn upload(&self, core: &GraphicsCore) {
        let m = self.calc_view_mat(core);
        let cambuf = self.make_cambuf(&m);
        self.buffer.write(&cambuf);
    }

    fn calc_view_mat(&self, core: &GraphicsCore) -> math2d::Matrix3x2f {
        use math2d::Matrix3x2f;

        let window_size = core.window().get_inner_size().unwrap();
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
}
