// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, May 2020

use crate::geometry::{Position, Size};

/// Brief description
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
    where T: PartialEq + Default
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
