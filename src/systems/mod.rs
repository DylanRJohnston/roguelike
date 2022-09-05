use legion::Schedule;

use self::{
    camera::camera_system, entity_render::entity_render_system, map_render::map_render_system,
    player_input::player_input_system,
};

mod camera;
mod entity_render;
mod map_render;
mod player_input;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input_system())
        .add_system(map_render_system())
        .add_system(camera_system())
        .add_system(entity_render_system())
        .build()
}
