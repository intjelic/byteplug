// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, May 2020

/// The drawing primitives
///
/// A drawing primitive is specified in conjunction with a set of vertices in order to define how
/// they are used to render shapes.
///
/// Note that points and lines have **no area**, therefore their thickness will always be 1 pixel,
/// regardless the current transform and view.
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Primitive {
    /// List of individual points.
    Points,
    /// List of individual lines.
    Lines,
    /// List of connected lines, a point uses the previous point to form a line.
    LineStrips,
    /// List of individual triangles.
    Triangles,
    /// List of connected triangles, a point uses the two previous points to form a triangle.
    TriangleStrips,
    /// List of connected triangles, a point uses the common center and the previous point to form a triangle.
    TriangleFans
}
