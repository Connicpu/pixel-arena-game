use crate::graphics::GraphicsState;

pub mod time;

#[derive(conniecs::ServiceManager)]
pub struct Services {
    pub quit_flag: bool,
    pub jump: bool,
    pub graphics: GraphicsState,
    pub time: time::Time,
    pub map: crate::tiled::map::Map,
}
