// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

//! Mathematical utilities related to geometry
//!
//! The geometry module provides the basics utilities to work in the Euclidean space, and is crafted
//! to go in pair with the draw module. It provides the base components that is used by almost all
//! the other modules such positions and sizes.
//!
//! It's a light version of highly mathematical oriented geometry crates out there, more affordable
//! for less mathematically-inclined programmers. It strips items from their complex mathematical
//! concepts that is either not relevant or hardly ever needed for our particular context.
//! **Module structure**
//!
//! Items in this module can be mentally divided into different categories.
//!
//! - Position, Size and Box items
//! - Vector and Matrix items
//! - Transform item
//! - Movable, Resizable, Rotable and Scalable items
//! - Point, Line, Rectangle, Circle and Polygon items
//!
//! Positions, sizes and boxes are the base components, used by almost all the other modules of this
//! crate; they are self-explanatory, except box, which simply is a rectangle (a position and a
//! size). Vectors and matrices are two mathematical primitives that allow to do useful geometrical
//! calculation such as transforming positions; they work together. Transform is a higher-level
//! primitive that encapsulates a matrix to transform position and boxes.
//! Movable, resizable, rotable and scalable are traits that user-defined items can implement when
//! it supports some common geometrical operations. Points, lines, rectangles, circles and polygons
//! items hold geometrical information and allow to compute collision detection.
//!
//! **Implementation status**
//!
//! - Traits (movable, resizable, rotable, scalable) aren't implemented yet and are subject to be
//!   removed.
//! - Trait `Transformable` still need to be defined; perhaps will be removed in the future.
//! - Points, lines, circles and rectangles aren't implemented yet. They're expected to have an
//!   interface to detect collisions.
//! - Polygon is to be added and implemented too.
//! - The `Box` item is likely to be renamed because it's clashing with the `box` Rust keyword.
//! - Many unit tests are missing.
//! - Implement `From` and `Into` trait to convert 2D coordinates-like items into each other (
//!   Position, Vector and perhaps Size).
//! - Implementation the `Scalar` trait or struct to solve few issues with the design. It would
//!   represent any valid built-in numeric types. It will allow to know what is the zero() version
//!   for item T constrained by the Scalar trait. It will also allow to implement vector and matrix
//!   multiplication with a scalar (implementing it for type T is conflicting with other stuff).
//!
mod utils;

mod position;
mod size;
mod box_;

mod vector;
mod matrix;

mod transform;
mod transformable;

mod point;
mod line;
mod rectangle;
mod circle;

pub use utils::compute_bounds;

pub use position::Position;
pub use size::Size;
pub use box_::Box;

pub use vector::Vector;
pub use matrix::Matrix;

pub use transform::Transform;
pub use transformable::Transformable;

pub use point::Point;
pub use line::Line;
pub use rectangle::Rectangle;
pub use circle::Circle;
