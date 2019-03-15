use crate::Comps;

pub use self::sprite::Sprite;
pub use self::transform::Transform;

pub mod shadow;
pub mod sprite;
pub mod transform;

#[rustfmt::skip]
#[derive(conniecs::ComponentManager)]
pub struct Components {
    #[hot] pub transform: Comps<transform::Transform>,
    #[hot] pub sprite: Comps<sprite::Sprite>,
    #[hot] pub shadow: Comps<shadow::Shadow>,

    #[cold] pub player: Comps<()>,
}
