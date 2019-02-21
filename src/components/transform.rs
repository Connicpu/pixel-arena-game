use math2d::Matrix3x2f as M;
use math2d::{Point2f, Vector2f};

#[derive(Copy, Clone, Debug)]
pub struct Transform {
    pub pos: Vector2f,
    pub scale: Vector2f,
    pub rotation: f32,
    pub skew: Vector2f,
    pub offset: Vector2f,
    pub sprite_center: Point2f,
    pub altitude: f32,
    pub z_layer: f32,
}

impl Transform {
    pub fn matrix(&self) -> M {
        M::skew(self.skew.x, self.skew.y, self.sprite_center)
            * M::rotation(self.rotation, self.sprite_center)
            * M::scaling(self.scale, self.sprite_center)
            * M::translation(self.pos + self.offset + [0.0, self.altitude])
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            pos: [0.0, 0.0].into(),
            scale: [1.0, 1.0].into(),
            rotation: 0.0,
            skew: [0.0, 0.0].into(),
            offset: [0.0, 0.0].into(),
            sprite_center: [0.0, 0.0].into(),
            altitude: 0.0,
            z_layer: 0.0,
        }
    }
}
