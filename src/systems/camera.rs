use bracket_lib::prelude::Point;
use legion::{component, system, world::SubWorld, IntoQuery};

use crate::{camera::Camera, components::Player};

#[system]
#[read_component(Point)]
#[read_component(Player)]
pub fn camera(ecs: &mut SubWorld, #[resource] camera: &mut Camera) {
    <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .for_each(|player| camera.update(player));
}
