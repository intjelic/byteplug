// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, May 2020

use crate::geometry::Position;
use crate::image::Color;

/// A 2D point with a color and a texture coordinate.
///
/// A vertex is an improved 2D point; other than its **position**, it also has a **color** and a
/// **texture coordinate**. It's the building block to draw anything on the screen (everything ever
/// drawn on screen is made of vertices). With a **set of vertices** and a given **drawing
/// primitive**, you are able to draw on a surface.
///
/// The position is defined by the `x` and `y` fields, the color is defined by the `r`, `g`, `b`
/// and `a` fields (which must be normalized values), and the texture coordinate is defined by the
/// `u` and `v` fields. Note that the `u` and `v` fields are unsigned integer because they
/// represent a **pixel coordinate** of a texture.
///
/// **Notes**
///
/// - I didn't spend much time on designing the interface; it's still fairly incomplete. It should
///   be done at a later stage when the overall interface of the framework is starting to be
///   fixed and stable.
///
#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Vertex {
    /// The X coordinate of the vertex.
    pub x: f32,
    /// The X coordinate of the vertex.
    pub y: f32,
    /// The red component of the vertex color.
    pub r: f32,
    /// The green component of the vertex color.
    pub g: f32,
    /// The blue component of the vertex color.
    pub b: f32,
    /// The alpha component of the vertex color.
    pub a: f32,
    /// The X coordinate of the texture.
    pub u: u32,
    /// The Y coordinate of the texture.
    pub v: u32
}

impl Vertex {
    /// Constructs a default vertex.
    ///
    /// The vertex has (0, 0) as position, black as color and (0, 0) as texture coordinate.
    ///
    pub fn new() -> Vertex {
        Vertex {
            x: 0.0, y: 0.0,
            r: 0.0, g: 0.0, b: 0.0, a: 1.0,
            u: 0, v: 0
        }
    }

    /// Constructs a vertex from a position and a color
    ///
    /// The vertex has the given position and color, with (0, 0) as texture coordinate.
    ///
    pub fn with_position_and_color(position: Position<f32>, color: Color) -> Vertex {
        Vertex {
            x: position.x as _,
            y: position.y as _,
            r: color.red   as f32 / 255.0,
            g: color.green as f32 / 255.0,
            b: color.blue  as f32 / 255.0,
            a: color.alpha as f32 / 255.0,
            u: 0,
            v: 0
        }
    }
}

impl Default for Vertex {
    fn default() -> Self {
        // The implementation of the default vertex is needed because the alpha value is 1.0, not
        // zero.
        Self::new()
    }
}
