use bracket_lib::prelude::{Point, VirtualKeyCode};
use legion::{component, system, world::SubWorld, IntoQuery};

use crate::{components::Player, models::map::Map};

#[allow(clippy::enum_variant_names)]
enum Intent {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

const fn intent(key: VirtualKeyCode) -> Option<Intent> {
    match key {
        VirtualKeyCode::Left | VirtualKeyCode::A => Some(Intent::MoveLeft),
        VirtualKeyCode::Right | VirtualKeyCode::D => Some(Intent::MoveRight),
        VirtualKeyCode::Up | VirtualKeyCode::W => Some(Intent::MoveUp),
        VirtualKeyCode::Down | VirtualKeyCode::S => Some(Intent::MoveDown),
        _ => None,
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
) {
    let mut try_move_player = |delta: Point| {
        <&mut Point>::query()
            .filter(component::<Player>())
            .iter_mut(ecs)
            .for_each(|player| {
                let new_position = *player + delta;

                if map.can_enter(new_position) {
                    *player = new_position;
                }
            });
    };

    let act = |intent: Intent| match intent {
        Intent::MoveUp => try_move_player(Point::new(0, -1)),
        Intent::MoveDown => try_move_player(Point::new(0, 1)),
        Intent::MoveLeft => try_move_player(Point::new(-1, 0)),
        Intent::MoveRight => try_move_player(Point::new(1, 0)),
    };

    key.and_then(intent).map(act);
}
