// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

/// Brief description
///
/// The **Vector struct** is not documented yet. Pull requests are welcome.
///
#[derive(Debug, Copy, Clone, PartialEq)] // implement Eq ?
pub struct Vector<T = f64> {
    pub radius: T,
    pub angle:  f64
}

impl<T> Vector<T> {
    pub fn new(radius: T, angle: f64) -> Vector<T> {
        Vector::<T> {
            radius: radius,
            angle:  angle
        }
    }
}
