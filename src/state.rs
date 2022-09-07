use bracket_lib::prelude::{Rect, *};
use legion::{Resources, Schedule, World};

use crate::{camera::Camera, models::map, spawn, systems};

pub struct State {
    pub ecs: World,
    pub resources: Resources,
    pub systems: Schedule,
}

impl State {
    pub fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();

        let map_builder = map::Builder::new(&mut rng).build();

        let camera = Camera::new(Point::new(0, 0), 40, 25);

        resources.insert(map_builder.map);
        resources.insert(camera);

        spawn::player(&mut ecs, map_builder.rooms[0].center());
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(Rect::center)
            .for_each(|pos| spawn::monster(&mut ecs, &mut rng, pos));

        Self {
            ecs,
            resources,
            systems: systems::build_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, terminal: &mut BTerm) {
        terminal.set_active_console(0);
        terminal.cls();
        terminal.set_active_console(1);
        terminal.cls();

        self.resources.insert(terminal.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);

        render_draw_buffer(terminal).expect("Render error");
    }
}
