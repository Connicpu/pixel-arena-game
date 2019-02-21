#![feature(manually_drop_take)]

#[macro_use]
extern crate conniecs_derive;
#[macro_use]
extern crate hex_literal;

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
type DataHelper = conniecs::DataHelper<Components, Services>;
//type EntityData<'a> = conniecs::EntityData<'a, components::Components>;

fn main() -> Result<(), failure::Error> {
    {
        use crate::tiled::raw::context::{ParseContext, Source};
        let mut tilesets = Default::default();
        let mut warnings = vec![];
        let config = Default::default();
        let mut map_ctx = ParseContext {
            reader: xml::EventReader::from_str(""),
            source: Source::new_file("assets/maps/placeholder/simple-grass-test.tmx"),
            tilesets: &mut tilesets,
            warnings: &mut warnings,
            config: &config,
        };
        println!("{}", map_ctx.source);
        let tsx = "../../tilesets/placeholder/simple-grass.tsx";
        println!("{}", map_ctx.source.relative(tsx));
        let tileset = tiled::raw::tileset::Tileset::parse_file(&mut map_ctx, tsx).unwrap();
        println!("{:#?}", tileset);
    }

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
