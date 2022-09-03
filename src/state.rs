use rltk::{to_cp437, GameState, Point, Rltk, VirtualKeyCode, BLACK, GREY50, WHITE, YELLOW};
use specs::{Join, World, WorldExt};

use crate::{
    camera::Camera,
    components::{Player, Position, Renderable},
    models::map::{Map, Tile},
};

pub struct State {
    pub ecs: World,
    pub map: Map,
}

impl State {
    fn run_systems(&mut self) {
        // let mut lw = LeftWalker {};
        // lw.run_now(&self.ecs);
        // self.ecs.maintain();
    }

    fn try_move_player(&mut self, delta: Point) {
        let mut positions = self.ecs.write_storage::<Position>();
        let mut players = self.ecs.write_storage::<Player>();

        for (_, Position(point)) in (&mut players, &mut positions).join() {
            let new_position = *point + delta;

            if self.map.can_enter(new_position) {
                *point = new_position;
            }
        }
    }

    fn player_input(&mut self, ctx: &mut Rltk) {
        match ctx.key {
            Some(VirtualKeyCode::Left) => self.try_move_player(Point { x: -1, y: 0 }),
            Some(VirtualKeyCode::Right) => self.try_move_player(Point { x: 1, y: 0 }),
            Some(VirtualKeyCode::Up) => self.try_move_player(Point { x: 0, y: -1 }),
            Some(VirtualKeyCode::Down) => self.try_move_player(Point { x: 0, y: 1 }),
            _ => {}
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        self.player_input(ctx);
        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let players = self.ecs.read_storage::<Player>();

        let mut camera = Camera::new(Point::new(0, 0), 40, 25);

        ctx.set_active_console(0);
        ctx.cls();

        for (_, Position(point)) in (&players, &positions).join() {
            camera = Camera::new(*point, 40, 25);

            camera
                .worldspace_view_iter()
                .filter_map(|point| Some((camera.to_camera_space(&point), self.map.at(point)?)))
                .for_each(|(Point { x, y }, tile)| match tile {
                    Tile::Floor => ctx.set(x, y, WHITE, BLACK, to_cp437('.')),
                    Tile::Wall => ctx.set(x, y, WHITE, BLACK, to_cp437('#')),
                });
        }

        ctx.set_active_console(1);
        ctx.cls();

        let renderables = self.ecs.read_storage::<Renderable>();

        for (Position(pos), render) in (&positions, &renderables).join() {
            let Point { x, y } = camera.to_camera_space(pos);

            ctx.set(x, y, WHITE, BLACK, render.glyph)
        }
    }
}
