// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use std::default::Default;
use std::cmp;
use std::ops::{Add, AddAssign};

/// Brief description
///
/// The **Color struct** is not documented yet. Pull requests are welcome.
///
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Color {
    pub red:   u8,
    pub green: u8,
    pub blue:  u8,
    pub alpha: u8
}

impl Color {
    pub fn new() -> Color {
        Color {
            red:   0,
            green: 0,
            blue:  0,
            alpha: 255
        }
    }

    pub fn rgb(red: u8, green: u8, blue: u8) -> Color {
        Color {
            red:   red,
            green: green,
            blue:  blue,
            alpha: 255
        }
    }

    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            red:   red,
            green: green,
            blue:  blue,
            alpha: alpha
        }
    }

    pub fn argb(alpha: u8, red: u8, green: u8, blue: u8) -> Color {
        Color {
            red:   red,
            green: green,
            blue:  blue,
            alpha: alpha
        }
    }

    pub const BLACK: Color = Color {
        red:   0,
        green: 0,
        blue:  0,
        alpha: 255
    };

    pub const WHITE: Color = Color {
        red:   255,
        green: 255,
        blue:  255,
        alpha: 255
    };

    pub const RED: Color = Color {
        red:   255,
        green: 0,
        blue:  0,
        alpha: 255
    };

    pub const GREEN: Color = Color {
        red:   0,
        green: 255,
        blue:  0,
        alpha: 255
    };

    pub const BLUE: Color = Color {
        red:   0,
        green: 0,
        blue:  255,
        alpha: 255
    };

    pub const YELLOW: Color = Color {
        red:   255,
        green: 255,
        blue:  0,
        alpha: 255
    };

    pub const MAGENTA: Color = Color {
        red:   255,
        green: 0,
        blue:  255,
        alpha: 255
    };

    pub const CYAN: Color = Color {
        red:   0,
        green: 255,
        blue:  255,
        alpha: 255
    };

    pub const TRANSPARENT: Color = Color {
        red:   0,
        green: 0,
        blue:  0,
        alpha: 0
    };
}

impl Default for Color {
    fn default() -> Color {
        Color::new()
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            // todo: can we retrieve the type of color components and use their maximum value ??
            red:   cmp::min(self.red as i32   + other.red as i32,   std::u8::MAX as i32) as u8,
            green: cmp::min(self.green as i32 + other.green as i32, std::u8::MAX as i32) as u8,
            blue:  cmp::min(self.blue as i32  + other.blue as i32,  std::u8::MAX as i32) as u8,
            alpha: cmp::min(self.alpha as i32 + other.alpha as i32, std::u8::MAX as i32) as u8
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}
