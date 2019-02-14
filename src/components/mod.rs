use crate::Comps;

pub use self::transform::Transform;
pub use self::sprite::Sprite;

pub mod transform;
pub mod sprite;

#[derive(ComponentManager)]
pub struct Components {
    #[hot]
    pub transform: Comps<transform::Transform>,

    #[cold]
    pub sprite: Comps<sprite::Sprite>,
}
