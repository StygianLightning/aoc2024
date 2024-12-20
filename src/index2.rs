use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Index2 {
    pub x: i32,
    pub y: i32,
}

impl Index2 {
    pub fn new(x: i32, y: i32) -> Self {
        Index2 { x, y }
    }

    pub fn zero() -> Index2 {
        Index2::new(0, 0)
    }

    pub fn to_index2(self) -> Option<UIndex2> {
        if self.x < 0 || self.y < 0 {
            return None;
        }

        Some(uidx2(self.x as _, self.y as _))
    }
}

impl Add for Index2 {
    type Output = Index2;

    fn add(self, rhs: Index2) -> Self::Output {
        Index2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Index2 {
    type Output = Index2;

    fn sub(self, rhs: Index2) -> Self::Output {
        Index2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<Index2> for i32 {
    type Output = Index2;

    fn mul(self, rhs: Index2) -> Self::Output {
        Index2::new(rhs.x * self, rhs.y * self)
    }
}

impl Div<i32> for Index2 {
    type Output = Index2;

    fn div(self, x: i32) -> Self::Output {
        Index2::new(self.x / x, self.y / x)
    }
}

impl AddAssign for Index2 {
    fn add_assign(&mut self, rhs: Index2) {
        *self = *self + rhs;
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Deserialize, Serialize)]
pub struct UIndex2 {
    pub x: u32,
    pub y: u32,
}

pub fn uidx2(x: u32, y: u32) -> UIndex2 {
    UIndex2::new(x, y)
}

impl UIndex2 {
    pub fn new(x: u32, y: u32) -> Self {
        UIndex2 { x, y }
    }

    pub fn zero() -> UIndex2 {
        UIndex2::new(0, 0)
    }

    pub fn to_index2(self) -> Index2 {
        Index2::new(self.x as i32, self.y as i32)
    }

    pub fn size(&self) -> u32 {
        self.x * self.y
    }
}

impl Add for UIndex2 {
    type Output = UIndex2;

    fn add(self, rhs: UIndex2) -> Self::Output {
        UIndex2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for UIndex2 {
    type Output = UIndex2;

    fn sub(self, rhs: UIndex2) -> Self::Output {
        UIndex2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<UIndex2> for u32 {
    type Output = UIndex2;

    fn mul(self, rhs: UIndex2) -> Self::Output {
        UIndex2::new(rhs.x * self, rhs.y * self)
    }
}

impl Div<u32> for UIndex2 {
    type Output = UIndex2;

    fn div(self, x: u32) -> Self::Output {
        UIndex2::new(self.x / x, self.y / x)
    }
}

impl AddAssign for UIndex2 {
    fn add_assign(&mut self, rhs: UIndex2) {
        *self = *self + rhs;
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Deserialize, Serialize)]
pub struct U64Index2 {
    pub x: u64,
    pub y: u64,
}

pub fn u64idx2(x: u64, y: u64) -> U64Index2 {
    U64Index2::new(x, y)
}

impl U64Index2 {
    pub fn new(x: u64, y: u64) -> Self {
        U64Index2 { x, y }
    }

    pub fn zero() -> U64Index2 {
        U64Index2::new(0, 0)
    }

    pub fn to_index2(self) -> Index2 {
        Index2::new(self.x as i32, self.y as i32)
    }

    pub fn size(&self) -> u64 {
        self.x * self.y
    }
}

impl Add for U64Index2 {
    type Output = U64Index2;

    fn add(self, rhs: U64Index2) -> Self::Output {
        U64Index2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for U64Index2 {
    type Output = U64Index2;

    fn sub(self, rhs: U64Index2) -> Self::Output {
        U64Index2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<U64Index2> for u64 {
    type Output = U64Index2;

    fn mul(self, rhs: U64Index2) -> Self::Output {
        U64Index2::new(rhs.x * self, rhs.y * self)
    }
}

impl Div<u64> for U64Index2 {
    type Output = U64Index2;

    fn div(self, x: u64) -> Self::Output {
        U64Index2::new(self.x / x, self.y / x)
    }
}

impl AddAssign for U64Index2 {
    fn add_assign(&mut self, rhs: U64Index2) {
        *self = *self + rhs;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let a = Index2::new(1, 2);
        let b = Index2::new(3, 4);
        assert_eq!(a + b, Index2::new(4, 6));
    }

    #[test]
    fn test_add_assign() {
        let mut a = Index2::new(1, 2);
        let b = Index2::new(3, 4);
        a += b;
        assert_eq!(a, Index2::new(4, 6));
    }

    #[test]
    fn test_div_index2() {
        assert_eq!(Index2::new(4, 6) / 2, Index2::new(2, 3));
    }
}
