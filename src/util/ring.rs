use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub struct Ring {
    lower_bound: i32,
    upper_bound: i32,
    value: i32,
}

impl From<i32> for Ring {
    fn from(value: i32) -> Self {
        Self {
            lower_bound: i32::MIN,
            upper_bound: i32::MAX,
            value,
        }
    }
}

impl From<Ring> for i32 {
    fn from(val: Ring) -> Self {
        val.value
    }
}

impl Add for Ring {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.set(self.get() + rhs.get())
    }
}

impl AddAssign for Ring {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Ring {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.set(self.get() - rhs.get())
    }
}

impl SubAssign for Ring {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Ring {
    pub const fn new(lower_bound: i32, upper_bound: i32) -> Self {
        Self {
            lower_bound,
            upper_bound,
            value: lower_bound,
        }
    }

    pub const fn set(&self, value: i32) -> Self {
        Self {
            value: (value - self.lower_bound).rem_euclid(self.upper_bound - self.lower_bound)
                + self.lower_bound,
            ..*self
        }
    }

    pub const fn get(&self) -> i32 {
        self.value
    }
}
