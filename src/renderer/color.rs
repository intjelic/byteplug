// Copyright (c) 2020 - BytePlug
//
// This source file is part of Distilled Multimedia Library which is released
// under the MIT license. Please refer to the LICENSE file that can be found
// at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

/// Brief description
///
/// Long decripiton.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Color {
    pub red:   u8,
    pub green: u8,
    pub blue:  u8,
    pub alpha: u8
}

impl Color {
    pub fn new() -> Color {
        Color{
            red:   0,
            green: 0,
            blue:  0,
            alpha: 255
        }
    }
}