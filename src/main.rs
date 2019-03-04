#![feature(manually_drop_take, range_contains, euclidean_division)]

#[macro_use]
extern crate conniecs_derive;
#[macro_use]
extern crate hex_literal;
#[macro_use]
extern crate serde_derive;

use failure::Fallible;

pub use components::Components;
pub use services::Services;
pub use systems::Systems;

pub mod assets;
pub mod components;
pub mod entities;
pub mod graphics;
pub mod services;
pub mod systems;
pub mod tiled;

mod helpers;

type World = conniecs::World<Systems>;
type Comps<T> = conniecs::ComponentList<Components, T>;
type EntityIter<'a> = conniecs::EntityIter<'a, Components>;
type Data = conniecs::DataHelper<Components, Services>;
//type EntityData<'a> = conniecs::EntityData<'a, components::Components>;

fn main() -> Fallible<()> {
    // Create core services
    let services = Services {
        graphics: graphics::GraphicsState::new()?,
        quit_flag: false,
        jump: false,
        time: services::time::Time::new(),
    };

    let mut world: World = conniecs::World::with_services(services);

    // Create some test entities
    for y in -3..=3 {
        for x in -5..=5 {
            use components::Transform;
            world.data.create_entity(|e, c, _s| {
                c.transform.add(
                    e,
                    Transform {
                        pos: [x as f32 * 1.4, y as f32 * 1.4].into(),
                        offset: [0.0, 0.5].into(),
                        ..Transform::default()
                    },
                );

                c.sprite.add(e, Default::default());
                c.shadow.add(e, Default::default());
            });
        }
    }

    while !world.data.services.quit_flag {
        world.update();
    }

    Ok(())
}
