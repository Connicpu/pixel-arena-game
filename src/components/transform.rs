use math2d::Vector2f;

#[derive(Copy, Clone, Debug)]
pub struct Transform {
    pub pos: Vector2f,
    pub scale: Vector2f,
    pub rotation: f32,
    pub altitude: f32,
    pub skew: Vector2f,
}
