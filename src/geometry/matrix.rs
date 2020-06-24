// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use std::ops::{Add, Sub, Mul};
use crate::geometry::Vector;

/// A matrix for the Euclidean plane.
///
/// A 2x3 matrix with a hidden row of homogenous coordinates (0, 0 and 1) and therefore behaves as
/// a 3x3 matrix. It's used in conjunction with vectors to transform 2D coordinates. It can
/// effectively represent any transformation in the Euclidean plane and be combined with other
/// matrices to represent successive transformations.
///
/// See it as a rectangular array of numbers made of 2 rows and 3 columns where each number is
/// called element. Elements are indexed from left to right, and top to bottom (note that OpenGL
/// use a different convention but it's able to compute an array compatible with OpenGL). It also
/// defines useful operations such a combine(), determinant(), inverse() and the expected arithmetic
/// operations with scalars, vectors and matrices.
///
/// Note that it's a primitive mathematical concept which can be hard to use and understand, in
/// practice, you use a `Transformer` which wraps a matrix and set its values for you.
///
/// **Implementation notes**
///
/// - Arithmetic operator overloads aren't implemented yet.
/// - The 3x3 and 4x4 array method changes the indices convention to be OpenGL compatible.
/// - It can't implement a `transpose()` since it's strictly a 2x3 matrix and it would change the
///   dimension.
///
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Matrix {
    pub elements: [f32; 6]
}

impl Matrix {

    /// Brief description
    ///
    /// The **new() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn new() -> Matrix {
        let mut elements: [f32; 6] = [0.0; 6];
        elements[0] = 1.0;
        elements[4] = 1.0;

        Self::with_elements(elements)
    }

    /// Brief description
    ///
    /// The **with_values() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn with_elements(elements: [f32; 6]) -> Matrix {
        Matrix {
            elements: elements
        }
    }

    /// Brief description
    ///
    /// The **combine() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn combine(&self, matrix: Matrix) -> Matrix {
        Matrix::with_elements([
            self.elements[0] * matrix.elements[0] + self.elements[1] * matrix.elements[3],
            self.elements[0] * matrix.elements[1] + self.elements[1] * matrix.elements[4],
            self.elements[0] * matrix.elements[2] + self.elements[1] * matrix.elements[5] + self.elements[2],
            self.elements[3] * matrix.elements[0] + self.elements[4] * matrix.elements[3],
            self.elements[3] * matrix.elements[1] + self.elements[4] * matrix.elements[4],
            self.elements[3] * matrix.elements[2] + self.elements[4] * matrix.elements[5] + self.elements[5],
        ])
    }

    /// Brief description
    ///
    /// The **determinant() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn determinant(&self) -> f32 {
        self.elements[0] * self.elements[4] - self.elements[1] * self.elements[3]
    }

    /// Brief description
    ///
    /// The **inverse() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn inverse(&self) -> Matrix {
        let determinant = self.determinant();

        // Compute the inverse if the determinant is not zero (don't use an epsilon because the
        // determinant may *really* be tiny).
        if determinant != 0.0 {
            Matrix::with_elements([
                 self.elements[4] / determinant,
                -self.elements[1] / determinant,
                 (self.elements[1] * self.elements[5] - self.elements[2] * self.elements[4]) / determinant,
                -self.elements[3] / determinant,
                 self.elements[0] / determinant,
                -(self.elements[0] * self.elements[5] - self.elements[2] * self.elements[3]) / determinant
            ])
        }
        else {
            Self::IDENTITY
        }
    }

    /// Brief description
    ///
    /// The **as_3x3_array() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn as_3x3_array(&self) -> [f32; 9] {
        let mut array = [0.0; 9];

        array[0] = self.elements[0];
        array[1] = self.elements[3];
        array[2] = 0.0;
        array[3] = self.elements[1];
        array[4] = self.elements[4];
        array[5] = 0.0;
        array[6] = self.elements[2];
        array[7] = self.elements[5];
        array[8] = 1.0;

        array
    }

    /// Brief description
    ///
    /// The **as_4x4_array() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn as_4x4_array(&self) -> [f32; 16] {
        let mut array = [0.0; 16];
        array[10] = 1.0;

        array[0]  = self.elements[0];
        array[4]  = self.elements[1];
        array[12] = self.elements[2];
        array[1]  = self.elements[3];
        array[5]  = self.elements[4];
        array[13] = self.elements[5];
        array[15] = 1.0;

        array
    }

    /// Brief description
    ///
    /// The **IDENTITY constant** is not documented yet. Pull requests are welcome.
    ///
    pub const IDENTITY: Matrix = Matrix {
        elements: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0]
    };
}

impl Add<Matrix> for Matrix {
    type Output = Self;

    fn add(self, rhs: Matrix) -> Self {
        Matrix::default()
    }
}

impl Sub<Matrix> for Matrix {
    type Output = Self;

    fn sub(self, rhs: Matrix) -> Self {
        Matrix::default()
    }
}

impl Mul<Vector> for Matrix {
    type Output = Self;

    fn mul(self, rhs: Vector) -> Self {
        Matrix::default()
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Self;

    fn mul(self, rhs: Matrix) -> Self {
        Matrix::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_new() {
        assert_eq!(Matrix::new(), Matrix::IDENTITY);
    }

    #[test]
    fn matrix_with_elements() {
        let elements = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        assert_eq!(Matrix::with_elements(elements).elements, elements);
    }

    #[test]
    fn matrix_combine() {
        let matrix = Matrix::with_elements([11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
        let other_matrix = Matrix::with_elements([21.0, 22.0, 23.0, 24.0, 25.0, 26.0]);
        let mut result = Matrix::default();

        result = matrix.combine(other_matrix);
        assert_eq!(result, Matrix::with_elements([519.0, 542.0, 578.0, 654.0, 683.0, 728.0]));

        result = other_matrix.combine(matrix);
        assert_eq!(result, Matrix::with_elements([539.0, 582.0, 648.0, 614.0, 663.0, 738.0]));
    }

    #[test]
    fn matrix_determinant() {
        let mut matrix = Matrix::default();

        matrix = Matrix::with_elements([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        assert_eq!(matrix.determinant(), -3.0);

        matrix = Matrix::with_elements([-6.0, -5.0, -4.0, -3.0, -2.0, -1.0]);
        assert_eq!(matrix.determinant(), -3.0);

        matrix = Matrix::with_elements([7.0, 8.0, 12.0, -8.0, -14.0, -10.0]);
        assert_eq!(matrix.determinant(), -34.0);
    }

    #[test]
    fn matrix_inverse() {
        let mut matrix = Matrix::default();

        let matrix = Matrix::with_elements([11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
        assert_eq!(matrix.inverse(), Matrix::with_elements([-5.0, 4.0, 1.0, 4.6666665, -3.6666667, -2.0]));

        let matrix = Matrix::with_elements([21.0, 22.0, 23.0, 24.0, 25.0, 26.0]);
        assert_eq!(matrix.inverse(), Matrix::with_elements([-8.333333, 7.3333335, 1.0, 8.0, -7.0, -2.0]));
    }

    #[test]
    fn matrix_identity() {
        let matrix = Matrix::IDENTITY;

        assert_eq!(matrix.elements[0], 1.0);
        assert_eq!(matrix.elements[1], 0.0);
        assert_eq!(matrix.elements[2], 0.0);
        assert_eq!(matrix.elements[3], 0.0);
        assert_eq!(matrix.elements[4], 1.0);
        assert_eq!(matrix.elements[5], 0.0);
    }

    #[test]
    fn matrix_addition() {
        // To be written.
        let matrix = Matrix::new();
        let another_matrix = Matrix::new();

        let result = matrix + another_matrix;
    }

    #[test]
    fn matrix_subtraction() {
        // To be written.
        let matrix = Matrix::new();
        let another_matrix = Matrix::new();

        let result = matrix - another_matrix;
    }

    #[test]
    fn matrix_multiplication() {
        let matrix = Matrix::new();
        let vector = Vector::from_xy(0.0, 0.0);
        let another_matrix = Matrix::new();

        let result_vector = matrix * vector;
        let result_matrix = matrix * another_matrix;
    }
}
