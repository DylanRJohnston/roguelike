use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

use specs::Component;
use specs::DenseVecStorage;
use specs_derive::Component;

use crate::util::Ring;

#[derive(Debug, Clone, Copy, Component)]
pub struct Position {
    pub x: Ring,
    pub y: Ring,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
