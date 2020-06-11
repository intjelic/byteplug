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
pub struct Vector {
    pub radius: f32,
    pub angle: f32
}

impl Vector {
    pub fn new(radius: f32, angle: f32) -> Vector {
        Vector {
            radius: radius,
            angle:  angle
        }
    }

    pub fn x(&self) -> f32 {
        self.angle.cos() * self.radius
    }

    pub fn y(&self) -> f32 {
        self.angle.sin() * self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_new() {
        // To be written.
    }

    #[test]
    fn vector_x() {
        // To be written.
    }

    #[test]
    fn vector_y() {
        // To be written.
    }
}
