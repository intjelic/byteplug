// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, June 2020

/// Brief description
///
/// Long description.
///
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum Uniform {
    Integer(i32),
    UnsignedInteger(u32),
    Float(f32),
    Vector2(f32, f32),
    Vector3(f32, f32, f32),
    Matrix2([f32; 4]),
    Matrix3([f32; 9]),
    Matrix4([f32; 16])
}
