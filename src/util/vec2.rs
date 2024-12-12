use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Vec2<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T: Add<Output=T>> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub<Output=T>> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Copy + Mul<Output=T>> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Copy + From<u8> + Ord> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 {
            x,
            y,
        }
    }

    pub fn zero() -> Vec2<T> {
        Vec2::new(T::from(0), T::from(0))
    }

    pub fn one() -> Vec2<T> {
        Vec2::new(T::from(1), T::from(1))
    }

    pub fn max(self, other: Self) -> Vec2<T> {
        Vec2 {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    pub fn min(self, other: Self) -> Vec2<T> {
        Vec2 {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn in_bounds(self, min: Self, max: Self) -> bool {
        self.x >= min.x && self.y >= min.y &&
            self.x <= max.x && self.y <= max.y
    }
}

// impl<T: Copy + Add + Mul> Vec2<T> {
//     pub fn len(self) -> T
//     where
//         <T as Mul>::Output: Add,
//     {
//         self.x * self.x + self.y * self.y
//     }
// }