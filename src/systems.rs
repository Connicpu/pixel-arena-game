use crate::graphics::systems as graphics;
use crate::services;

#[derive(conniecs::SystemManager)]
pub struct Systems {
    update_time: services::time::UpdateTime,

    begin_draw: graphics::BeginDraw,
    render_sprites: graphics::RenderSprites,
    render_shadows: graphics::RenderShadows,
    end_draw: graphics::EndDraw,
    
    window_events: graphics::WindowEvents,
}