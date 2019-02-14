use crate::graphics::GraphicsState;

#[derive(ServiceManager)]
pub struct Services {
    pub graphics: GraphicsState,
}
