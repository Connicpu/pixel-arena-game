pub type WindowEvents = window_events::WindowEvents;
pub type BeginDraw = begin_draw::BeginDraw;
pub type EndDraw = end_draw::EndDraw;
pub type RenderSprites = conniecs::EntitySystem<render_sprites::RenderSprites>;

pub mod begin_draw;
pub mod end_draw;
pub mod render_sprites;
pub mod window_events;
