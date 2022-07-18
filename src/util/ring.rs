use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub struct Ring {
    lower_bound: i32,
    upper_bound: i32,
    value: i32,
}

impl From<i32> for Ring {
    fn from(value: i32) -> Self {
        Ring {
            lower_bound: i32::MIN,
            upper_bound: i32::MAX,
            value,
        }
    }
}

impl Into<i32> for Ring {
    fn into(self) -> i32 {
        self.value
    }
}

impl Add for Ring {
    type Output = Ring;

    fn add(self, rhs: Self) -> Self::Output {
        self.set(self.get() + rhs.get())
    }
}

impl AddAssign for Ring {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for Ring {
    type Output = Ring;

    fn sub(self, rhs: Self) -> Self::Output {
        self.set(self.get() - rhs.get())
    }
}

impl SubAssign for Ring {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Ring {
    pub fn new(lower_bound: i32, upper_bound: i32) -> Self {
        Ring {
            lower_bound,
            upper_bound,
            value: lower_bound,
        }
    }

    pub fn set(&self, value: i32) -> Self {
        Ring {
            value: (value - self.lower_bound).rem_euclid(self.upper_bound - self.lower_bound)
                + self.lower_bound,
            ..*self
        }
    }

    pub fn get(&self) -> i32 {
        self.value
    }
}
