#![feature(range_contains, euclidean_division)]

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
pub mod input;
pub mod physics;
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

    let mut box2d = physics::World::new(&[0.0, -10.0].into());
    let map = {
        use crate::tiled::source::Source;

        let src = Source::new_file("assets/maps/placeholder/simple-grass-test.tmx");
        let mut map = tiled::load_tmx(src)?;

        map.tilesets.initialize(&graphics.core)?;

        for layer in map.layers.iter_mut() {
            if let tiled::map::layer::Layer::Tile(layer) = layer {
                for (cpos, chunk) in layer.data.chunks.iter_mut() {
                    use tiled::map::tiledata::CHUNK_SIZE;
                    let pos =
                        (cpos.to_f32().to_vector() * [1.0, -1.0] * CHUNK_SIZE as f32).to_point();
                    chunk.create_physics(&map.tilesets, &pos, &mut box2d);
                }
            }
        }

        map
    };

    // Create core services
    let services = Services {
        graphics,
        quit_flag: false,
        jump: false,
        time: services::time::Time::new(),
        map,
        box2d,
    };

    let mut world: World = conniecs::World::with_services(services);

    world.data.create_entity(|e, c, s| {
        use crate::components::Transform;

        let transform = Transform {
            scale: [0.5, 0.5].into(),
            offset: [0.0, 0.25].into(),
            z_layer: 1.0,
            ..Default::default()
        };

        let tm = &mut s.graphics.textures;
        let core = &s.graphics.core;
        let sprite = tm.load_simple("characters/playertemp.png", core).unwrap();

        c.transform.add(e, transform);
        c.sprite.add(e, sprite);
        c.shadow.add(e, Default::default());
        c.player.add(e, ());
    });

    while !world.data.services.quit_flag {
        world.update();
    }

    Ok(())
}
