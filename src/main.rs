#![feature(
    manually_drop_take,
    range_contains,
    euclidean_division,
    core_intrinsics
)]

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
    let graphics = graphics::GraphicsState::new()?;

    let map = {
        use crate::tiled::source::Source;

        let src = Source::new_file("assets/maps/placeholder/simple-grass-test.tmx");
        let mut map = tiled::load_tmx(src)?;

        map.tilesets.initialize(&graphics.core)?;

        map
    };

    // Create core services
    let services = Services {
        graphics,
        quit_flag: false,
        jump: false,
        time: services::time::Time::new(),
        map,
    };

    let mut world: World = conniecs::World::with_services(services);

    // Create some test entities
    // for y in -3..=3 {
    //     for x in -5..=5 {
    //         use components::Transform;
    //         world.data.create_entity(|e, c, _s| {
    //             c.transform.add(
    //                 e,
    //                 Transform {
    //                     pos: [x as f32 * 1.4, y as f32 * 1.4].into(),
    //                     offset: [0.0, 0.5].into(),
    //                     ..Transform::default()
    //                 },
    //             );

    //             c.sprite.add(e, Default::default());
    //             c.shadow.add(e, Default::default());
    //         });
    //     }
    // }

    while !world.data.services.quit_flag {
        world.update();
    }

    Ok(())
}
