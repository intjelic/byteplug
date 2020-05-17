// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

//! Mathematical utilities related to geometry
//!
//! Additional documentation is to be written here.
mod position;
mod size;
mod box_;
mod vector;

mod transform;

mod movable;
mod rotable;
mod scalable;

mod resizable;

mod transformable;

mod point;
mod line;
mod rectangle;
mod circle;

pub use position::Position;
pub use size::Size;
pub use box_::Box;
pub use vector::Vector;

pub use transform::Transform;

pub use movable::Movable;
pub use rotable::Rotable;
pub use scalable::Scalable;

pub use resizable::Resizable;

pub use transformable::Transformable;

pub use point::Point;
pub use line::Line;
pub use rectangle::Rectangle;
pub use circle::Circle;
