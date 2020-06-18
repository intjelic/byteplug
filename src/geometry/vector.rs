// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use std::ops::{Add, Sub, Mul};
use crate::geometry::Matrix;

/// A vector for the Euclidean plane.
///
/// A vector represents **a length** and **an angle**; it can be seen as an arrow. With matrices,
/// it's one of the two mathematical primitives used to compute in **the Euclidean plane**; it's
/// mostly used to transform points. Also note that, unlike the mathematical literature, vectors are
/// not directly a one dimension matrix (or a "row vector" or a "column vector"), even though they
/// can be thought in that way.
///
/// **Implementation notes**
///
/// - Rename `radius` to `length` later.
/// - Implement unit vector constant.
/// - Implement in-place arithmetic operation equivalents (AddAssign, SubAssign, MulAssign)
/// - Implement scalar and update to allow multiplication with scalar.
/// - Think about `normalize()` versus `normalized()`.
///
#[derive(Copy, Clone, PartialEq, Debug, Default)]
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

    pub fn normalize(&mut self) {
        // To be implemented.
    }

    pub fn dot_product(&mut self) {
        // To be implemented.
    }

    pub fn cross_product(&mut self) {
        // To be implemented.
    }
}

impl Add<Vector> for Vector {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self {
        Vector::default()
    }
}

impl Sub<Vector> for Vector {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self {
        Vector::default()
    }
}

impl Mul<Vector> for Vector {
    type Output = Self;

    fn mul(self, rhs: Vector) -> Self {
        Vector::default()
    }
}

impl Mul<Matrix> for Vector {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        Matrix::default()
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

    #[test]
    fn vector_normalize() {
        // To be written.
    }

    #[test]
    fn vector_dot_product() {
        // To be written.
    }

    #[test]
    fn vector_cross_product() {
        // To be written.
    }

    #[test]
    fn vector_addition() {
        // To be written.
        let vector = Vector::default();
        let another_vector = Vector::default();

        let result = vector + another_vector;
    }

    #[test]
    fn vector_subtraction() {
        // To be written.
        let vector = Vector::default();
        let another_vector = Vector::default();

        let result = vector - another_vector;
    }

    #[test]
    fn vector_multiplication() {
        // To be written.
        let vector = Vector::default();
        let another_vector = Vector::default();
        let matrix = Matrix::new();

        let result_vector = vector * another_vector;
        let result_matrix = vector * matrix;
    }
}
