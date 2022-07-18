use specs::{Join, ReadStorage, System, WriteStorage};

use crate::components::{LeftMover, Position};

pub struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            *pos -= Position::new(1, 0);
        }
    }
}
