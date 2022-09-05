use bracket_lib::prelude::{to_cp437, ColorPair, Point, RandomNumberGenerator, BLACK, WHITE};
use legion::World;

use crate::components::{enemy::Enemy, Player, Renderable};

pub fn player(ecs: &mut World, position: Point) {
    ecs.push((
        Player {},
        position,
        Renderable {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

pub fn monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    ecs.push((
        Enemy,
        pos,
        Renderable {
            color: ColorPair::new(WHITE, BLACK),
            glyph: match rng.range(0, 4) {
                0 => to_cp437('E'),
                1 => to_cp437('O'),
                2 => to_cp437('o'),
                _ => to_cp437('g'),
            },
        },
    ));
}
