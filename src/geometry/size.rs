// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

/// A two-dimensional size.
///
/// The **Size struct** is not documented yet. Pull requests are welcome.
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Size<T = f32> {
    pub width:  T,
    pub height: T
}

impl<T> Size<T>
    where T: PartialEq + Default
{

    /// Brief description
    ///
    /// The **new() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn new(width: T, height: T) -> Size<T> {
        Size::<T> {
            width: width,
            height: height
        }
    }

    /// Brief description
    ///
    /// The **zero() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn zero() -> Size<T> {
        Size::new(T::default(), T::default())
    }

    /// Brief description
    ///
    /// The **is_zero() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn is_zero(&self) -> bool {
        self.width == T::default() && self.height == T::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_new() {
        let size = Size::new(1, 2);

        assert_eq!(size.width, 1);
        assert_eq!(size.height, 2);
    }

    #[test]
    fn size_zero() {
        let size: Size<f32> = Size::zero();

        assert_eq!(size.width, 0.0);
        assert_eq!(size.height, 0.0);
    }

    #[test]
    fn size_is_zero() {
        let mut size: Size<f32> = Size::zero();
        assert_eq!(size.is_zero(), true);

        size.width = 1.0;
        size.height = 0.0;
        assert_eq!(size.is_zero(), false);

        size.width = 0.0;
        size.height = -1.0;
        assert_eq!(size.is_zero(), false);
    }
}
