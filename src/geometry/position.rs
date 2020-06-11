// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::ops::{Mul, MulAssign, Div, DivAssign};
use std::ops::Neg;

/// Brief description
///
/// The **Position struct** is not documented yet. Pull requests are welcome.
///
/// **Implementation notes**
///
/// - The implementation of Position<T> will be completed later.
/// - Just like we should eagerly implement the common traits (as per the Rust library guidelines)
///   we should also implement common operators where it makes sense.
/// - My first implementation of the multiplication and division traits involved a second generic
///   type U; this was to support multiplication and division with a type different from the type of
///   the position. But then the other operations (addition and subtraction) should also implement
///   this... it's a bit complicated to figure out what is the right Rust way to do this as it
///   involves few concepts... casting, numeric types and coercions, etc. For now, I keep everything
///   simple.
/// - Divisions and multiplication should support two types.
/// - Addition and multiplication are commutative, but I wasn't able to figure out how to implement
///   this, or figure out if it's a good practice.
/// - Implementation of operator traits seem generic. Isn't there a standard macro to implement
///   those automatically ?
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Position<T = f32> {
    pub x: T,
    pub y: T
}

impl<T> Position<T> {
    pub fn new(x: T, y: T) -> Position<T> {
        Position::<T> {
            x: x,
            y: y
        }
    }
}

impl<T: Add<Output = T>> Add for Position<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<T: Copy + Add<Output = T>> AddAssign for Position<T> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y
        };
    }
}

impl<T: Sub<Output = T>> Sub for Position<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl<T: Copy + Sub<Output = T>> SubAssign for Position<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y
        };
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Position<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other
        }
    }
}

impl<T: Copy + Mul<Output = T>> MulAssign<T> for Position<T> {
    fn mul_assign(&mut self, other: T) {
        *self = Self {
            x: self.x * other,
            y: self.y * other
        };
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Position<T> {
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other
        }
    }
}

impl<T: Copy + Div<Output = T>> DivAssign<T> for Position<T> {
    fn div_assign(&mut self, other: T) {
        *self = Self {
            x: self.x / other,
            y: self.y / other
        };
    }
}

impl<T: Neg<Output = T>> Neg for Position<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_add() {
        let a = Position::new(1, 2);
        let b = Position::new(3, 4);

        let mut c = a + b;
        assert_eq!(c, Position::new(4, 6));

        let d = Position::new(5, 6);
        c += d;
        assert_eq!(c, Position::new(9, 12));
    }

    #[test]
    fn position_subtract() {
        let a = Position::new(1, 2);
        let b = Position::new(3, 4);

        let mut c = a - b;
        assert_eq!(c, Position::new(-2, -2));

        let d = Position::new(5, 6);
        c -= d;
        assert_eq!(c, Position::new(-7, -8));
    }

    #[test]
    fn position_multiplication() {
        let a = Position::new(1, 2);

        let b = a * 2;
        assert_eq!(b, Position::new(2, 4));

        let mut c = Position::new(5, 6);
        c *= 2;
        assert_eq!(c, Position::new(10, 12));
    }

    #[test]
    fn position_division() {
        let a = Position::new(2, 4);

        let b = a / 2;
        assert_eq!(b, Position::new(1, 2));

        let mut c = Position::new(10, 12);
        c /= 2;
        assert_eq!(c, Position::new(5, 6));
    }

    #[test]
    fn position_negation() {
        let a = Position::new(1, -2);
        let b = -a;
        assert_eq!(b, Position::new(-1, 2));
    }
}
