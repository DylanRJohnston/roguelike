mod components;
mod state;
mod systems;
mod util;

use components::{LeftMover, Player, Position, Renderable};
use rltk::{main_loop, to_cp437, BResult, RltkBuilder, RGB};
use specs::{Builder, World, WorldExt};
use state::State;
use util::Ring;

fn main() -> BResult<()> {
    let terminal = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut ecs = World::new();
    ecs.register::<Position>();
    ecs.register::<Renderable>();
    ecs.register::<LeftMover>();
    ecs.register::<Player>();

    let coordinates = Position {
        x: Ring::new(0, 80),
        y: Ring::new(0, 50),
    };

    ecs.create_entity()
        .with(coordinates + Position::new(35, 20))
        .with(Renderable {
            glyph: to_cp437('☺'),
            foreground: RGB::named(rltk::YELLOW),
            background: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    for i in 1..10 {
        ecs.create_entity()
            .with(coordinates + Position::new(i * 7, 25))
            .with(Renderable {
                glyph: to_cp437('@'),
                foreground: RGB::named(rltk::RED),
                background: RGB::named(rltk::BLACK),
            })
            .with(LeftMover {})
            .build();
    }

    main_loop(terminal, State { ecs })
}
