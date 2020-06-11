// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use std::f32::consts::PI;
use crate::geometry::Position;
use crate::geometry::Box;
use crate::geometry::{Vector, Matrix};

/// Brief description
///
/// The **Transform struct** is not documented yet. Pull requests are welcome.
///
/// **Implementation notes**
///
/// - The translate(), rotate(), scale() and combine() methods consume and return `self`; is that
///   good design ?
/// - Is combine() method necessary ?
///
pub struct Transform {
    pub matrix: Matrix
}

impl Transform {
    /// Brief description
    ///
    /// The **new() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn new() -> Transform {
        Transform {
            matrix: Matrix::new()
        }
    }

    /// Brief description
    ///
    /// The **with_matrix() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn with_matrix(matrix: Matrix) -> Transform {
        Transform {
            matrix: matrix
        }
    }

    /// Brief description
    ///
    /// The **translate() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn translate(mut self, offset: Vector) -> Transform {
        let matrix = Matrix::with_values([
            1.0, 0.0, offset.x(),
            0.0, 1.0, offset.y(),
            0.0, 0.0, 1.0,
        ]);

        self.matrix = self.matrix.combine(&matrix);
        self
    }

    /// Brief description
    ///
    /// The **rotate() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn rotate(mut self, angle: f32, center: Option<Position>) -> Transform {

        let radian = angle * PI / 180.0;
        let cosine = radian.cos();
        let sine = radian.sin();

        let matrix = match center {
            Some(position) => {
                Matrix::with_values([
                    cosine, -sine,   position.x * (1.0 - cosine) + position.y * sine,
                    sine,    cosine, position.y * (1.0 - cosine) - position.x * sine,
                    0.0,     0.0,    1.0,
                ])
            },
            None => {
                Matrix::with_values([
                    cosine, -sine,   0.0,
                    sine,    cosine, 0.0,
                    0.0,     0.0,    1.0,
                ])
            }
        };

        self.matrix = self.matrix.combine(&matrix);
        self
    }

    /// Brief description
    ///
    /// The **scale() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn scale(mut self, factor: Vector, center: Option<Position>) -> Transform {

        let matrix = match center {
            Some(position) => {
                Matrix::with_values([
                    factor.x(), 0.0,        position.x * (1.0 - factor.x()),
                    0.0,        factor.y(), position.y * (1.0 - factor.y()),
                    0.0,        0.0,        1.0
                ])
            },
            None => {
                Matrix::with_values([
                    factor.x(), 0.0,        0.0,
                    0.0,        factor.y(), 0.0,
                    0.0,        0.0,        1.0
                ])
            }
        };

        self.matrix = self.matrix.combine(&matrix);
        self
    }

    // /// Brief description
    ///
    /// The **combine() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn combine(mut self, matrix: &Matrix) -> Transform {
        self.matrix = self.matrix.combine(matrix);
        self
    }

    /// Brief description
    ///
    /// The **transform_position() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn transform_position(&self, position: &Position<f32>) -> Position<f32> {
        Position::new(
            self.matrix[0] * position.x + self.matrix[1] * position.y + self.matrix[2],
            self.matrix[3] * position.x + self.matrix[4] * position.y + self.matrix[5]
        )
    }

    /// Brief description
    ///
    /// The **transform_box() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn transform_box(&self, boxx: &Box<f32>) -> Box<f32> {

        // // Transform the 4 corners of the rectangle
        // const Vector2f points[] =
        // {
        //     transformPoint(rectangle.left, rectangle.top),
        //     transformPoint(rectangle.left, rectangle.top + rectangle.height),
        //     transformPoint(rectangle.left + rectangle.width, rectangle.top),
        //     transformPoint(rectangle.left + rectangle.width, rectangle.top + rectangle.height)
        // };

        // // Compute the bounding rectangle of the transformed points
        // float left = points[0].x;
        // float top = points[0].y;
        // float right = points[0].x;
        // float bottom = points[0].y;
        // for (int i = 1; i < 4; ++i)
        // {
        //     if      (points[i].x < left)   left = points[i].x;
        //     else if (points[i].x > right)  right = points[i].x;
        //     if      (points[i].y < top)    top = points[i].y;
        //     else if (points[i].y > bottom) bottom = points[i].y;
        // }

        // return FloatRect(left, top, right - left, bottom - top);

        Box::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_new() {
        assert_eq!(Transform::new().matrix, Matrix::IDENTITY);
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
    fn transform_scale() {
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
