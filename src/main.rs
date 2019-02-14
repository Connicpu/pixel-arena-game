#![feature(manually_drop_take)]

#[macro_use]
extern crate conniecs_derive;

pub use components::Components;
pub use services::Services;

pub mod components;
pub mod entities;
pub mod graphics;
pub mod services;
pub mod systems;

type Comps<T> = conniecs::ComponentList<components::Components, T>;
type EntityIter<'a> = conniecs::EntityIter<'a, components::Components>;
type DataHelper = conniecs::DataHelper<components::Components, services::Services>;
//type EntityData<'a> = conniecs::EntityData<'a, components::Components>;

fn main() -> Result<(), failure::Error> {
    // Create graphics core
    let mut graphics = graphics::GraphicsState::new()?;

    println!("{:?}", graphics.core.window.get_inner_size());

    graphics.core.events_loop.run_forever(|e| {
        use winit::ControlFlow::{Break, Continue};
        use winit::{Event::WindowEvent, WindowEvent::CloseRequested};
        
        if let WindowEvent { event, .. } = e {
            match event {
                CloseRequested => Break,
                _ => Continue,
            }
        } else {
            Continue
        }
    });

    Ok(())
}
