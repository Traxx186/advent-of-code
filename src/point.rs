use std::ops::{AddAssign, Add, Sub, Mul};

use num::Signed;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl <T: Signed + Sub<Output = T> + Copy> Point2D<T> {
    pub fn sigsum(&self, other: &Self) -> Point2D<T> {
        Self { 
            x: (self.x - other.x).signum(),
            y: (self.y - other.y).signum(),
        }
    }
}

impl <T> From<(T, T)> for Point2D<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl <T: Add<Output = T> + Copy> AddAssign for Point2D<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl <T: Add<Output = T>> Add for Point2D<T> {
    type Output = Point2D<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x, 
            y: self.y + rhs.y,
        }
    }
}

impl <T: Sub<Output = T>> Sub for Point2D<T> {
    type Output = Point2D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl <T: Mul<Output = T>> Mul for Point2D<T> {
    type Output = Point2D<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point3D<T> {
        pub x: T,
        pub y: T,
        pub z: T,
}

impl <T: Signed + Sub<Output = T> + Copy> Point3D<T> {
    pub fn sigsum(&self, other: &Self) -> Point3D<T> {
        Self { 
            x: (self.x - other.x).signum(),
            y: (self.y - other.y).signum(),
            z: (self.z - other.z).signum()
        }
    }
}

impl <T> From<(T, T, T)> for Point3D<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self { x, y, z }
    }
}

impl <T: Add<Output = T> + Copy> AddAssign for Point3D<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl <T: Add<Output = T>> Add for Point3D<T> {
    type Output = Point3D<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x, 
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl <T: Sub<Output = T>> Sub for Point3D<T> {
    type Output = Point3D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl <T: Mul<Output = T>> Mul for Point3D<T> {
    type Output = Point3D<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
