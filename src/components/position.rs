use rltk::Point;
use specs::Component;
use specs::DenseVecStorage;
use specs_derive::Component;

#[derive(Debug, Clone, Copy, Component)]
pub struct Position(pub Point);
