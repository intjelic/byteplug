// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, June 2020

use crate::geometry::Position;
use crate::geometry::Matrix;
use crate::geometry::Transformer;

/// Brief description.
///
/// The **Transformable struct** is not documented yet. Pull requests are welcome.
///
pub trait Transformable {
    fn position(&self) -> Position;
    fn set_position(&mut self, position: Position);

    fn angle(&self) -> f32;
    fn set_angle(&mut self, angle: f32);

    fn magnifier(&self) -> f32;
    fn set_magnifier(&mut self, magnifier: f32);

    fn translate(&mut self, offset: Position) {
        self.set_position(self.position() + offset);
    }

    fn rotate(&mut self, angle: f32) {
        self.set_angle(self.angle() + angle);
    }

    fn magnify(&mut self, factor: f32) {
        self.set_magnifier(self.magnifier() * factor);
    }

    fn matrix(&self) -> Matrix {
        let transformer = Transformer::new()
            .translate(self.position())
            .rotate(self.angle(), None)
            .magnify(self.magnifier(), None);

        transformer.matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transformable_tests() {
        // To be implemented.
    }
}
