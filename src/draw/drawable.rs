// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, June 2020

use crate::draw::Surface;

/// Brief description.
///
/// The **Drawable struct** is not documented yet. Pull requests are welcome.
///
pub trait Drawable {
    fn draw(&self, surface: &mut Surface);
}
