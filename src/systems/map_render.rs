use bracket_lib::prelude::{to_cp437, ColorPair, DrawBatch, Point, RGBA, WHITE};
use legion::system;

use crate::{
    camera::Camera,
    models::map::{Curve, Direction, Map, Tile},
};

const BACKGROUND: RGBA = RGBA {
    r: 140.0 / 255.0,
    g: 176.0 / 255.0,
    b: 155.0 / 255.0,
    a: 1.0,
};

const UP: Point = Point { x: 0, y: -1 };
const DOWN: Point = Point { x: 0, y: 1 };

const COLOR: ColorPair = ColorPair {
    fg: RGBA {
        r: WHITE.0 as f32 / 255.0,
        g: WHITE.1 as f32 / 255.0,
        b: WHITE.2 as f32 / 255.0,
        a: 1.0,
    },
    bg: BACKGROUND,
};

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut basic_batch = DrawBatch::new();
    basic_batch.target(0);

    let mut corner_batch = DrawBatch::new();
    corner_batch.target(0);

    camera
        .worldspace_view_iter()
        .filter_map(|point| Some((camera.to_camera_space(point), map.at(point)?)))
        .for_each(|(pos, tile)| {
            match tile {
                Tile::Void => {
                    basic_batch.set(pos, COLOR, to_cp437('$'));
                }
                Tile::Floor => {
                    basic_batch.set(pos, COLOR, to_cp437('.'));
                }
                // Close corners

                // Convex Walls
                // 1 2
                // A B
                // Q R
                // a b
                Tile::Wall(Curve::Convex, Direction::NE) => {
                    corner_batch.set(pos, COLOR, to_cp437('2'));
                    corner_batch.set(pos + DOWN, COLOR, to_cp437('B'));
                }
                Tile::Wall(Curve::Convex, Direction::NW) => {
                    corner_batch.set(pos + DOWN, COLOR, to_cp437('A'));
                    corner_batch.set(pos, COLOR, to_cp437('1'));
                }
                Tile::Wall(Curve::Convex, Direction::SE) => {
                    corner_batch.set(pos, COLOR, to_cp437('b'));
                    corner_batch.set(pos + UP, COLOR, to_cp437('R'));
                }
                Tile::Wall(Curve::Convex, Direction::SW) => {
                    corner_batch.set(pos, COLOR, to_cp437('a'));
                    corner_batch.set(pos + UP, COLOR, to_cp437('Q'));
                }
                Tile::Wall(Curve::Concave, Direction::NW) => {
                    corner_batch.set(pos, COLOR, to_cp437('X'));
                    corner_batch.set(pos + UP, COLOR, to_cp437('H'));
                }
                Tile::Wall(Curve::Concave, Direction::N) => {
                    basic_batch.set(pos, COLOR, to_cp437('Y'));
                    basic_batch.set(pos + UP, COLOR, to_cp437('I'));
                }
                Tile::Wall(Curve::Concave, Direction::NE) => {
                    corner_batch.set(pos, COLOR, to_cp437('Z'));
                    corner_batch.set(pos + UP, COLOR, to_cp437('J'));
                }
                Tile::Wall(Curve::Concave, Direction::W) => {
                    basic_batch.set(pos, COLOR, to_cp437('h'));
                }
                Tile::Wall(Curve::Concave, Direction::E) => {
                    basic_batch.set(pos, COLOR, to_cp437('j'));
                }
                Tile::Wall(Curve::Concave, Direction::SW) => {
                    corner_batch.set(pos, COLOR, to_cp437('x'));
                }
                Tile::Wall(Curve::Concave, Direction::S) => {
                    basic_batch.set(pos, COLOR, to_cp437('y'));
                }
                Tile::Wall(Curve::Concave, Direction::SE) => {
                    corner_batch.set(pos, COLOR, to_cp437('z'));
                }
                Tile::Wall(..) => {}
            };
        });

    basic_batch.submit(0).expect("Batch Error");
    corner_batch.submit(1000).expect("Batch Error");
}

// Void  -> $
// Wall Font Top  -> #
// Wall Front Bottom -> %
// Floor -> .

// H I J
// X Y Z
// h i j
// x y z
