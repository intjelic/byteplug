// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

/// Brief description
///
/// The **Position struct** is not documented yet. Pull requests are welcome.
///
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position<T = i32> {
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
