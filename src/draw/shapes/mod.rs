// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

//! The shapes sub-module
//!
//! Additional documentation is to be written here.
mod point;
mod line;
mod triangle;
mod rectangle;
mod circle;
mod polygon;

pub use point::Point;
pub use line::Line;
pub use triangle::Triangle;
pub use rectangle::Rectangle;
pub use circle::Circle;
pub use polygon::Polygon;
