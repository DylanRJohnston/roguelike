use bracket_lib::prelude::{DrawBatch, Point};
use legion::{system, world::SubWorld, IntoQuery};

use crate::{camera::Camera, components::Renderable};

#[system]
#[read_component(Point)]
#[read_component(Renderable)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    <(&Point, &Renderable)>::query()
        .iter(ecs)
        .for_each(|(position, renderable)| {
            draw_batch.set(
                camera.to_camera_space(position),
                renderable.color,
                renderable.glyph,
            );
        });

    draw_batch.submit(5000).expect("Batch error");
}
