// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, May 2020

use std::ops::Add;
use crate::geometry::{Position, Size};

/// A two-dimensional box.
///
/// The **Box struct** is not documented yet. Pull requests are welcome.
///
/// **Implementation notes**
///
/// - The interface and implementation will change a lot.
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Box<T = f32>
{
    pub position: Position<T>,
    pub size: Size<T>
}

impl<T> Box<T>
    where T: Copy + PartialEq + Default + Add<Output=T>
{
    /// Brief description
    ///
    /// The **new() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn new(position: Position<T>, size: Size<T>) -> Box<T> {
        Box::<T> {
            position: position,
            size: size
        }
    }

    /// Brief description
    ///
    /// The **zero() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn zero() -> Box<T> {
        Box::new(Position::zero(), Size::zero())
    }

    /// Brief description
    ///
    /// The **is_zero() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn is_zero(&self) -> bool {
        self.position == Position::zero() && self.size == Size::zero()
    }

    /// Brief description
    ///
    /// The **top_left() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn top_left(&self) -> Position<T> {
        self.position
    }

    /// Brief description
    ///
    /// The **top() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn top(&self) -> T {
        self.position.y
    }

    /// Brief description
    ///
    /// The **top_right() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn top_right(&self) -> Position<T> {
        Position::new(self.position.x + self.size.width, self.position.y)
    }

    /// Brief description
    ///
    /// The **left() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn left(&self) -> T {
        self.position.x
    }

    // /// Brief description
    // ///
    // /// The **center() function** is not documented yet. Pull requests are welcome.
    // ///
    // pub fn center(&self) -> Position<T> {
    //     Position::new(
    //         self.position.x + self.size.width / 2,
    //         self.position.y + self.size.height / 2
    //     )
    // }

    /// Brief description
    ///
    /// The **right() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn right(&self) -> T {
        self.position.x + self.size.width
    }

    /// Brief description
    ///
    /// The **bottom_left() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn bottom_left(&self) -> Position<T> {
        Position::new(self.position.x, self.position.y + self.size.height)
    }

    /// Brief description
    ///
    /// The **bottom() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn bottom(&self) -> T {
        self.position.y + self.size.height
    }

    /// Brief description
    ///
    /// The **bottom_right() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn bottom_right(&self) -> Position<T> {
        Position::new(self.position.x + self.size.width, self.position.y + self.size.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_new() {
        let box_ = Box::new(Position::new(1, 2), Size::new(3, 4));

        assert_eq!(box_.position, Position::new(1, 2));
        assert_eq!(box_.size, Size::new(3, 4));
    }

    #[test]
    fn box_zero() {
        let box_: Box<f32> = Box::zero();

        assert_eq!(box_.position, Position::zero());
        assert_eq!(box_.size, Size::zero());
    }

    #[test]
    fn box_is_zero() {
        let mut box_: Box<f32> = Box::zero();
        assert_eq!(box_.is_zero(), true);

        box_.position.x = 1.0;
        box_.position.y = 0.0;
        box_.size.width = 0.0;
        box_.size.height = 0.0;
        assert_eq!(box_.is_zero(), false);

        box_.position.x = 0.0;
        box_.position.y = 0.0;
        box_.size.width = 0.0;
        box_.size.height = -1.0;
        assert_eq!(box_.is_zero(), false);
    }
}
