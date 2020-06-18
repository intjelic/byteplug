// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use std::f32::consts::PI;
use crate::geometry::{Position, Size, Box};
use crate::geometry::{Vector, Matrix};

/// Brief description
///
/// The **View struct** is not documented yet. Pull requests are welcome.
///
/// **Implementation notes**
///
/// - The viewport() and reset() methods aren't implemented; do they need to be implemented ?
/// - Constructor from_box() was favored over from_size() as we can do from_box(Pos::zero, size)
/// - I don't like the reset() method but I implemented it for now.
/// - The SFML-like viewport concept doesn't exist yet.
/// - Unit tests arent implemented yet.
/// - The `move()` method conflicts with Rust keyword.
/// - The `move()`, `set_rotation()` and `zoom()` methods aren't implemented.
///
#[derive(Clone)]
pub struct View {
    center: Position<f32>,
    size: Size<f32>,
    rotation: f32,
    matrix: Matrix,
    update: bool
}

impl View {

    /// Brief description
    ///
    /// The **new() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn new(center: Position, size: Size) -> View {
        View {
            center: center,
            size: size,
            rotation: 0.0,
            matrix: Matrix::new(),
            update: true
        }
    }

    /// Brief description
    ///
    /// The **new() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn with_box(box_: Box) -> View {
        // todo; to be replaced with box_.center() as soon as it's implemented
        let mut center = Position::default();
        center.x = box_.position.x + box_.size.width / 2.0;
        center.y = box_.position.y + box_.size.height / 2.0;

        View {
            center: center,
            size: box_.size,
            rotation: 0.0,
            matrix: Matrix::new(),
            update: true
        }
    }

    /// Brief description
    ///
    /// The **center() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn center(&self) -> Position {
        self.center
    }

    /// Brief description
    ///
    /// The **set_center() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn set_center(&mut self, center: Position) {
        self.center = center;
        self.update = true;
    }

    /// Brief description
    ///
    /// The **size() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn size(&self) -> Size {
        self.size
    }

    /// Brief description
    ///
    /// The **set_size() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn set_size(&mut self, size: Size) {
        self.size = size;
        self.update = true;
    }

    /// Brief description
    ///
    /// The **rotation() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    /// Brief description
    ///
    /// The **set_rotation() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn set_rotation(&mut self, rotation: f32) {
        // todo; implement equivalent of the following piece of code
        // m_rotation = static_cast<float>(fmod(angle, 360));
        // if (m_rotation < 0)
        //     m_rotation += 360.f;

        self.rotation = rotation;
        self.update = true;
    }

    /// Brief description
    ///
    /// The **reset() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn reset(&mut self, box_: Box) {
        // todo; to be adjusted as soon as center() in Box is implemented.

        // self.center = box_.center();
        self.center.x = box_.position.x + box_.size.width / 2.0;
        self.center.y = box_.position.y + box_.size.height / 2.0;

        self.size = box_.size;
        self.rotation = 0.0;

        self.update = true;
    }

    /// Brief description
    ///
    /// The **move() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn move_(&mut self, offset: Vector) {
        // To be written.
    }

    /// Brief description
    ///
    /// The **rotate() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn rotate(&mut self, angle: f32) {
        self.set_rotation(self.rotation + angle);
    }

    /// Brief description
    ///
    /// The **zoom() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn zoom(&mut self, factor: f32) {
        // To be written.
    }

    /// Brief description
    ///
    /// The **transform() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn matrix(&mut self) -> Matrix {

        // Recompute the matrix if needed.
        if self.update {

            // Rotation components.
            let angle  = self.rotation * PI / 180.0;
            let cosine = angle.cos();
            let sine   = angle.sin();

            let tx = -self.center.x * cosine - self.center.y * sine   + self.center.x;
            let ty =  self.center.x * sine   - self.center.y * cosine + self.center.y;

            // Projection components.
            let a =  2.0 / self.size.width;
            let b = -2.0 / self.size.height;
            let c = -a * self.center.x;
            let d = -b * self.center.y;

            // Rebuild the projection matrix.
            self.matrix = Matrix::with_values([
                 a * cosine, a * sine,   a * tx + c,
                -b * sine,   b * cosine, b * ty + d,
                 0.0,        0.0,        1.0
            ]);

            self.update = false;
        };

        self.matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn view_tests() {
        // To be written.
    }
}
