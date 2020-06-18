// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use std::ops::{Add, Sub, Mul};
use std::ops::{Index, IndexMut};
use crate::geometry::Vector;

/// A matrix for the Euclidean plane.
///
/// A matrix represents a rectangular array of numbers made of 3 rows and 3 columns (aka **3x3
/// **matrix). With vectors, it's one of the two mathematical primitives used to compute in **the
/// Euclidean plane**; it's mostly used to transform points. Matrices are a raw mathematical
/// concept, in practice, it's used with a `Transform` which operates on a matrix in order to
/// transform positions.
///
/// **Implementation notes**
///
/// - It's a partial copy of the SFML implementation of `sf::Transform`.
/// - Internally, it's a 4x4 matrix, but is it necessary ? Or is it just because it's passed down
///   to OpenGL which works in 3D.
/// - Arithmetic operator overloads aren't implemented yet.
/// - Should there be as_array() to return raw 9 or 16 long arrays ? See private `as_array()` method
///   below.
///
#[derive(Copy, Clone, Debug, Default)]
pub struct Matrix {
    values: [f32; 16]
}

impl Matrix {

    /// Brief description
    ///
    /// The **new() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn new() -> Matrix {
        let mut values: [f32; 16] = [0.0; 16];

        values[0] = 1.0; values[4] = 0.0; values[8]  = 0.0; values[12] = 0.0;
        values[1] = 0.0; values[5] = 1.0; values[9]  = 0.0; values[13] = 0.0;
        values[2] = 0.0; values[6] = 0.0; values[10] = 1.0; values[14] = 0.0;
        values[3] = 0.0; values[7] = 0.0; values[11] = 0.0; values[15] = 1.0;

        Matrix {
            values: values
        }
    }

    /// Brief description
    ///
    /// The **with_values() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn with_values(values_: [f32; 9]) -> Matrix {
        let mut values: [f32; 16] = [0.0; 16];

        values[0] = values_[0]; values[4] = values_[1]; values[8]  = 0.0; values[12] = values_[2];
        values[1] = values_[3]; values[5] = values_[4]; values[9]  = 0.0; values[13] = values_[5];
        values[2] = 0.0;        values[6] = 0.0;        values[10] = 1.0; values[14] = 0.0;
        values[3] = values_[6]; values[7] = values_[7]; values[11] = 0.0; values[15] = values_[8];

        Matrix {
            values: values
        }
    }

    pub fn values(&self) -> [f32; 9] {
        [
            self.values[0],
            self.values[4],
            self.values[12],
            self.values[1],
            self.values[5],
            self.values[13],
            self.values[3],
            self.values[7],
            self.values[15]
        ]
    }

    /// Brief description
    ///
    /// The **combine() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn combine(&self, matrix: &Matrix) -> Matrix {
        Matrix::with_values([
            self.values[0] * matrix.values[0]  + self.values[4] * matrix.values[1]  + self.values[12] * matrix.values[3],
            self.values[0] * matrix.values[4]  + self.values[4] * matrix.values[5]  + self.values[12] * matrix.values[7],
            self.values[0] * matrix.values[12] + self.values[4] * matrix.values[13] + self.values[12] * matrix.values[15],
            self.values[1] * matrix.values[0]  + self.values[5] * matrix.values[1]  + self.values[13] * matrix.values[3],
            self.values[1] * matrix.values[4]  + self.values[5] * matrix.values[5]  + self.values[13] * matrix.values[7],
            self.values[1] * matrix.values[12] + self.values[5] * matrix.values[13] + self.values[13] * matrix.values[15],
            self.values[3] * matrix.values[0]  + self.values[7] * matrix.values[1]  + self.values[15] * matrix.values[3],
            self.values[3] * matrix.values[4]  + self.values[7] * matrix.values[5]  + self.values[15] * matrix.values[7],
            self.values[3] * matrix.values[12] + self.values[7] * matrix.values[13] + self.values[15] * matrix.values[15]
        ])
    }

    /// Brief description
    ///
    /// The **inverse() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn inverse(&self) -> Matrix {
        // Compute the determinant.
        let determinant = self.values[0] * (self.values[15] * self.values[5] - self.values[7] * self.values[13]) -
                          self.values[1] * (self.values[15] * self.values[4] - self.values[7] * self.values[12]) +
                          self.values[3] * (self.values[13] * self.values[4] - self.values[5] * self.values[12]);

        // Compute the inverse if the determinant is not zero (don't use an epsilon because the
        // determinant may *really* be tiny).
        if determinant != 0.0
        {
            Matrix::with_values([
                 (self.values[15] * self.values[5] - self.values[7] * self.values[13]) / determinant,
                -(self.values[15] * self.values[4] - self.values[7] * self.values[12]) / determinant,
                 (self.values[13] * self.values[4] - self.values[5] * self.values[12]) / determinant,
                -(self.values[15] * self.values[1] - self.values[3] * self.values[13]) / determinant,
                 (self.values[15] * self.values[0] - self.values[3] * self.values[12]) / determinant,
                -(self.values[13] * self.values[0] - self.values[1] * self.values[12]) / determinant,
                 (self.values[7]  * self.values[1] - self.values[3] * self.values[5])  / determinant,
                -(self.values[7]  * self.values[0] - self.values[3] * self.values[4])  / determinant,
                 (self.values[5]  * self.values[0] - self.values[1] * self.values[4])  / determinant
            ])
        }
        else
        {
            Self::IDENTITY
        }
    }

    /// Brief description
    ///
    /// The **IDENTITY constant** is not documented yet. Pull requests are welcome.
    ///
    pub const IDENTITY: Matrix = Matrix {
        values: [
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0
        ]
    };

    pub(crate) fn as_array(&self) -> &[f32; 16] {
        &self.values
    }
}

fn index_9_to_16(index: usize) -> usize {
    match index {
        0 => 0,
        1 => 4,
        2 => 12,
        3 => 1,
        4 => 5,
        5 => 13,
        6 => 3,
        7 => 7,
        8 => 15,
        _ => panic!("invalid index")
    }
}

impl Index<usize> for Matrix {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index_9_to_16(index)]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index_9_to_16(index)]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.values() == other.values()
    }
}

impl Eq for Matrix {}

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
    fn matrix_with_values() {
        let values = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        assert_eq!(Matrix::with_values(values).values(), values);
    }

    #[test]
    fn matrix_values() {
        let values = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        assert_eq!(Matrix::with_values(values).values(), values);
    }

    #[test]
    fn matrix_index() {
        let values = [0.0, 2.0, 3.0, 4.0, 0.0, 6.0, 7.0, 8.0, 0.0];
        let mut matrix = Matrix::with_values(values);

        assert_eq!(matrix[0], 0.0);
        assert_eq!(matrix[1], 2.0);
        assert_eq!(matrix[2], 3.0);
        assert_eq!(matrix[3], 4.0);
        assert_eq!(matrix[4], 0.0);
        assert_eq!(matrix[5], 6.0);
        assert_eq!(matrix[6], 7.0);
        assert_eq!(matrix[7], 8.0);
        assert_eq!(matrix[8], 0.0);

        matrix[0] = 1.0;
        matrix[4] = 1.0;
        matrix[8] = 1.0;

        assert_eq!(matrix[0], 1.0);
        assert_eq!(matrix[1], 2.0);
        assert_eq!(matrix[2], 3.0);
        assert_eq!(matrix[3], 4.0);
        assert_eq!(matrix[4], 5.0);
        assert_eq!(matrix[5], 6.0);
        assert_eq!(matrix[6], 7.0);
        assert_eq!(matrix[7], 8.0);
        assert_eq!(matrix[8], 9.0);
    }

    #[test]
    fn matrix_combine() {
        // To be written.
    }

    #[test]
    fn matrix_inverse() {
        // To be written.
    }

    #[test]
    fn matrix_identity() {
        let identity = Matrix::IDENTITY;

        assert_eq!(identity[0], 1.0);
        assert_eq!(identity[1], 0.0);
        assert_eq!(identity[2], 0.0);
        assert_eq!(identity[3], 0.0);
        assert_eq!(identity[4], 1.0);
        assert_eq!(identity[5], 0.0);
        assert_eq!(identity[6], 0.0);
        assert_eq!(identity[7], 0.0);
        assert_eq!(identity[8], 1.0);
    }

    #[test]
    fn matrix_comparison() {
        // To be written.
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
        let vector = Vector::new(0.0, 0.0);
        let another_matrix = Matrix::new();

        let result_vector = matrix * vector;
        let result_matrix = matrix * another_matrix;
    }
}
