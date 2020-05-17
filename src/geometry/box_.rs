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
pub struct Box<T = i32> {
    pub position: Position<T>,
    pub size: Size<T>
}

impl<T> Box<T> {
    pub fn new(position: Position<T>, size: Size<T>) -> Box<T> {
        Box::<T> {
            position: position,
            size: size
        }
    }
}
