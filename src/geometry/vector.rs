// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use std::ops::Mul;

/// A vector for the Euclidean plane.
///
/// A 2D vector with a hidden homogenous coordinates (z equal to 0) and therefore behaves as a 3D
/// vector. It's used in conjunction with matrices to transform 2D coordinates. It can effectively
/// represent coordinates or directions in the Euclidean plane that are later transformed with
/// matrices.
///
/// See it as an arrow which has the characteristics of having an angle and a length, but which is
/// more commonly described with two values, x and y, which is preferred as the angle and the length
/// can easily be computed from those values.
///
/// It also defines useful operations such as normalize() to obtain a unit vector, the dot product
/// and the cross product (even if the later one isn't rigorously defined in the Euclidean plane),
/// and it doesn't implement the expected arithmetic operations with scalars, vectors and matrices
/// because they would not preserve the homogenous coordinate or result in a matrix with a different
/// dimension. The only arithmetic operation implemented is the vector-vector multiplication, which
/// is the same as using the dot_product() method.
///
/// Note that you should prefer using `Position<T>` to represent positions unless you want to
/// transform the coordinates in which case you should use a `Vector` (both are easily convertible
/// into one another).
///
/// **Implementation notes**
///
/// - Implement in-place arithmetic operation equivalents (AddAssign, SubAssign, MulAssign)
/// - Implement scalar and update to allow multiplication with scalar.
/// - Think about `normalize()` versus `normalized()`.
///
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Vector {
    pub x: f32,
    pub y: f32
}

impl Vector {

    /// Brief description
    ///
    /// The **new() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn new() -> Vector {
        Vector {
            x: 1.0,
            y: 0.0
        }
    }

    /// Brief description
    ///
    /// The **from_xy() function** is not documented yet. Pull requests are welcome.
    ///
    ///
    pub fn from_xy(x: f32, y: f32) -> Vector {
        Vector {
            x: x,
            y: y
        }
    }

    /// Brief description
    ///
    /// The **normalize4() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn normalize(&mut self) {
        // To be implemented.
    }

    /// Brief description
    ///
    /// The **dot_product() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn dot_product(&self, vector: Vector) -> f32 {
        self.x * vector.x + self.y * vector.y
    }

    /// Brief description
    ///
    /// The **cross_product() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn cross_product(&self, vector: Vector) -> f32 {
        // To be implemented.
    }
}

impl Mul<Vector> for Vector {
    type Output = f32;

    fn mul(self, vector: Vector) -> f32 {
        self.dot_product(vector)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_new() {
        let vector = Vector::new();

        assert_eq!(vector.x, 1.0);
        assert_eq!(vector.y, 0.0);
    }

    #[test]
    fn vector_from_xy() {
        let vector = Vector::from_xy(0.0, 1.0);

        assert_eq!(vector.x, 0.0);
        assert_eq!(vector.y, 1.0);
    }

    #[test]
    fn vector_normalize() {
        // To be written.
    }

    #[test]
    fn vector_dot_product() {
        let vector = Vector::from_xy(1.0, 2.0);
        let other_vector = Vector::from_xy(5.0, 7.0);
        assert_eq!(vector.dot_product(other_vector), 19.0);
        assert_eq!(other_vector.dot_product(vector), 19.0);
    }

    #[test]
    fn vector_cross_product() {
        // To be written.
    }

    #[test]
    fn vector_multiplication() {
        let vector = Vector::from_xy(1.0, 2.0);
        let other_vector = Vector::from_xy(5.0, 7.0);

        assert_eq!(vector * other_vector, vector.dot_product(other_vector));
        assert_eq!(other_vector * vector, other_vector.dot_product(vector));
    }
}
