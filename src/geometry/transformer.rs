// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use std::f32::consts::PI;
use crate::geometry::compute_bounds;
use crate::geometry::Position;
use crate::geometry::Box;
use crate::geometry::Matrix;

/// A position and rectangle transformer.
///
/// The **Transform struct** is not documented yet. Pull requests are welcome.
///
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Transformer {
    pub matrix: Matrix
}

impl Transformer {
    /// Brief description
    ///
    /// The **new() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn new() -> Transformer {
        Transformer::with_matrix(Matrix::new())
    }

    /// Brief description
    ///
    /// The **with_matrix() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn with_matrix(matrix: Matrix) -> Transformer {
        Transformer {
            matrix: matrix
        }
    }

    /// Brief description
    ///
    /// The **translate() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn translate(mut self, offset: Position) -> Transformer {
        let matrix = Matrix::with_elements([
            1.0, 0.0, offset.x,
            0.0, 1.0, offset.y
        ]);

        self.matrix = self.matrix.combine(matrix);
        self
    }

    /// Brief description
    ///
    /// The **rotate() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn rotate(mut self, angle: f32, center: Option<Position>) -> Transformer {
        let radian = angle * PI / 180.0;
        let cosine = radian.cos();
        let sine = radian.sin();

        let matrix = match center {
            Some(position) => {
                Matrix::with_elements([
                    cosine, -sine,   position.x * (1.0 - cosine) + position.y * sine,
                    sine,    cosine, position.y * (1.0 - cosine) - position.x * sine
                ])
            },
            None => {
                Matrix::with_elements([
                    cosine, -sine,   0.0,
                    sine,    cosine, 0.0
                ])
            }
        };

        self.matrix = self.matrix.combine(matrix);
        self
    }

    /// Brief description
    ///
    /// The **magnify() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn magnify(mut self, factor: f32, center: Option<Position>) -> Transformer {
        let matrix = match center {
            Some(position) => {
                Matrix::with_elements([
                    factor, 0.0,    position.x * (1.0 - factor),
                    0.0,    factor, position.y * (1.0 - factor)
                ])
            },
            None => {
                Matrix::with_elements([
                    factor, 0.0,    0.0,
                    0.0,    factor, 0.0
                ])
            }
        };

        self.matrix = self.matrix.combine(matrix);
        self
    }

    /// Brief description
    ///
    /// The **reset() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn reset(&mut self) {
        self.matrix = Matrix::IDENTITY;
    }

    /// Brief description
    ///
    /// The **transform_position() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn transform_position(&self, position: &Position<f32>) -> Position<f32> {
        Position::new(
            self.matrix.elements[0] * position.x + self.matrix.elements[1] * position.y + self.matrix.elements[2],
            self.matrix.elements[3] * position.x + self.matrix.elements[4] * position.y + self.matrix.elements[5]
        )
    }

    /// Brief description
    ///
    /// The **transform_box() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn transform_box(&self, box_: &Box<f32>) -> Box<f32> {

        // Transform the 4 corners of the box.
        let points = vec!(
            self.transform_position(&box_.top_left()),
            self.transform_position(&box_.top_right()),
            self.transform_position(&box_.bottom_left()),
            self.transform_position(&box_.bottom_right())
        );

        // Compute the bounding box of the transformed points.
        compute_bounds(&points)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_new() {
        assert_eq!(Transformer::new().matrix, Matrix::IDENTITY);
    }

    #[test]
    fn transform_with_matrix() {
        // To be written.
    }

    #[test]
    fn transform_translate() {
        // To be written.
    }

    #[test]
    fn transform_rotate() {
        // To be written.
    }

    #[test]
    fn transform_magnify() {
        // To be written.
    }

    #[test]
    fn transform_transform_position() {
        // To be written.
    }


    #[test]
    fn transform_transform_box() {
        // To be written.
    }
}
