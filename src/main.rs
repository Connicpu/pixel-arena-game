#![feature(manually_drop_take)]

#[macro_use]
extern crate conniecs_derive;

pub use components::Components;
pub use services::Services;
pub use systems::Systems;

pub mod components;
pub mod entities;
pub mod graphics;
pub mod services;
pub mod systems;

type World = conniecs::World<Systems>;
type Comps<T> = conniecs::ComponentList<Components, T>;
type EntityIter<'a> = conniecs::EntityIter<'a, Components>;
type DataHelper = conniecs::DataHelper<Components, Services>;
//type EntityData<'a> = conniecs::EntityData<'a, components::Components>;

fn main() -> Result<(), failure::Error> {
    // Create core services
    let services = Services {
        graphics: graphics::GraphicsState::new()?,
        quit_flag: false,
    };

    let mut world: World = conniecs::World::with_services(services);

    for y in -3..=3 {
        for x in -5..=5 {
            use components::Transform;
            world.data.create_entity(|e, c, _s| {
                c.transform.add(e, Transform {
                    pos: [x as f32 * 1.4, y as f32 * 1.4].into(),
                    ..Transform::default()
                });
                
                c.sprite.add(e, Default::default());
            });
        }
    }
    
    while !world.data.services.quit_flag {
        world.update();
    }

    Ok(())
}
