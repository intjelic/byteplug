// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

/// Brief description
///
/// The **Size struct** is not documented yet. Pull requests are welcome.
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Size<T = f32> {
    pub width:  T,
    pub height: T
}

impl<T> Size<T> {
    pub fn new(width: T, height: T) -> Size<T> {
        Size::<T> {
            width: width,
            height: height
        }
    }
}
