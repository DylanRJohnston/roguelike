use bracket_lib::prelude::{to_cp437, ColorPair, DrawBatch, BLACK, WHITE};
use legion::system;

use crate::{
    camera::Camera,
    models::map::{Map, Tile},
};

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    camera
        .worldspace_view_iter()
        .filter_map(|point| Some((camera.to_camera_space(&point), map.at(point)?)))
        .for_each(|(pos, tile)| {
            match tile {
                Tile::Floor => draw_batch.set(pos, ColorPair::new(WHITE, BLACK), to_cp437('.')),
                Tile::Wall => draw_batch.set(pos, ColorPair::new(WHITE, BLACK), to_cp437('#')),
            };
        });

    draw_batch.submit(0).expect("Batch Error");
}
