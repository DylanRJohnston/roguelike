mod components;
mod models;
mod state;
mod systems;
mod util;

use components::{LeftMover, Player, Position, Renderable};
use models::map::{self, Map, MapBuilder};
use rltk::{main_loop, to_cp437, BResult, Point, RandomNumberGenerator, RltkBuilder, RGB};
use specs::{Builder, World, WorldExt};
use state::State;

fn main() -> BResult<()> {
    let terminal = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .with_fps_cap(30.0)
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
            glyph: to_cp437('☺'),
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
