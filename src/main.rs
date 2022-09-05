#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod camera;
mod components;
mod models;
mod spawn;
mod state;
mod systems;
mod util;

use std::panic;

use bracket_lib::prelude::*;

use state::State;

const DUNGEONFONT: &[u8] = include_bytes!("../resources/dungeonfont.png");

fn main() -> BResult<()> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    EMBED
        .lock()
        .add_resource("../resources/dungeonfont.png".to_string(), DUNGEONFONT);

    let terminal = BTermBuilder::new()
        .with_title("Roguelike Tutorial")
        .with_fps_cap(30.0)
        .with_dimensions(40, 25)
        .with_tile_dimensions(32, 32)
        .with_resource_path("../resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(40, 25, "dungeonfont.png")
        .with_simple_console_no_bg(40, 25, "dungeonfont.png")
        .build()?;

    let state = State::new();

    main_loop(terminal, state)
}
