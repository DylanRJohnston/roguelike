mod camera;
mod components;
mod models;
mod state;
mod systems;
mod util;

use std::panic;

use components::{LeftMover, Player, Position, Renderable};
use models::map::{self, Map, MapBuilder};
use rltk::{
    embedded_resource, link_resource, main_loop, to_cp437, BResult, Point, RandomNumberGenerator,
    RltkBuilder, EMBED, RGB,
};
use specs::{Builder, World, WorldExt};
use state::State;

const dungeonfont: &'static [u8] = include_bytes!("../resources/dungeonfont.png");

fn main() -> BResult<()> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    EMBED
        .lock()
        .add_resource("../resources/dungeonfont.png".to_string(), dungeonfont);

    let terminal = RltkBuilder::new()
        .with_title("Roguelike Tutorial")
        .with_fps_cap(30.0)
        .with_dimensions(40, 25)
        .with_tile_dimensions(32, 32)
        .with_resource_path("../resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(40, 25, "dungeonfont.png")
        .with_simple_console_no_bg(40, 25, "dungeonfont.png")
        .build()?;

    let mut ecs = World::new();
    ecs.register::<Position>();
    ecs.register::<Renderable>();
    ecs.register::<LeftMover>();
    ecs.register::<Player>();

    let mut map_builder = MapBuilder {
        map: Map::new(),
        rooms: vec![],
        player_start: Point::new(0, 0),
        max_rooms: 20,
        rng: RandomNumberGenerator::new(),
    };

    map_builder.build_random_rooms();
    map_builder.dig_random_tunnels();

    ecs.create_entity()
        .with(Position(map_builder.rooms[10].center()))
        .with(Renderable {
            glyph: to_cp437('@'),
            foreground: RGB::named(rltk::GREEN),
            background: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    main_loop(
        terminal,
        State {
            ecs,
            map: map_builder.map,
        },
    )
}
