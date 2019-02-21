use crate::graphics::systems as graphics;

#[derive(SystemManager)]
pub struct Systems {
    begin_draw: graphics::BeginDraw,
    render_sprites: graphics::RenderSprites,
    render_shadows: graphics::RenderShadows,
    end_draw: graphics::EndDraw,
    
    window_events: graphics::WindowEvents,
}