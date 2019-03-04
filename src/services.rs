use crate::graphics::GraphicsState;

pub mod time;

#[derive(ServiceManager)]
pub struct Services {
    pub quit_flag: bool,
    pub jump: bool,
    pub graphics: GraphicsState,
    pub time: time::Time,
}
