use rltk::{to_cp437, GameState, Point, Rltk, VirtualKeyCode, BLACK, GREEN, GREY1, GREY50, YELLOW};
use specs::{Join, RunNow, World, WorldExt};

use crate::{
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
        ctx.cls();

        self.player_input(ctx);
        self.run_systems();

        self.map
            .coordinate_iter()
            .for_each(|(Point { x, y }, tile)| match tile {
                Tile::Floor => ctx.set(x, y, YELLOW, BLACK, to_cp437('.')),
                Tile::Wall => ctx.set(x, y, GREY50, BLACK, to_cp437('#')),
            });

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (Position(pos), render) in (&positions, &renderables).join() {
            ctx.set(
                pos.x,
                pos.y,
                render.foreground,
                render.background,
                render.glyph,
            )
        }
    }
}
