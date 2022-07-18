use rltk::{GameState, Rltk, VirtualKeyCode};
use specs::{Join, RunNow, World, WorldExt};

use crate::{
    components::{Player, Position, Renderable},
    systems::LeftWalker,
};

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker {};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }

    fn try_move_player(&mut self, delta: Position) {
        let mut positions = self.ecs.write_storage::<Position>();
        let mut players = self.ecs.write_storage::<Player>();

        for (_, pos) in (&mut players, &mut positions).join() {
            *pos += delta;
        }
    }

    fn player_input(&mut self, ctx: &mut Rltk) {
        match ctx.key {
            Some(VirtualKeyCode::Left) => self.try_move_player(Position::new(-1, 0)),
            Some(VirtualKeyCode::Right) => self.try_move_player(Position::new(1, 0)),
            Some(VirtualKeyCode::Up) => self.try_move_player(Position::new(0, -1)),
            Some(VirtualKeyCode::Down) => self.try_move_player(Position::new(0, 1)),
            _ => {}
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        self.player_input(ctx);
        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
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
