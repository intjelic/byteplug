// Copyright (c) 2020 - BytePlug
//
// This source file is part of Distilled Multimedia Library which is released
// under the MIT license. Please refer to the LICENSE file that can be found
// at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

//! The renderer module
//!
//! Additional documentation is to be written here.
include!("color.rs");
include!("gradient.rs");

include!("view.rs");
include!("surface.rs");

include!("renderable.rs");
include!("renderer.rs");

pub mod shapes {
    include!("shapes/point.rs");
    include!("shapes/line.rs");
    include!("shapes/triangle.rs");
    include!("shapes/rectangle.rs");
    include!("shapes/circle.rs");
    include!("shapes/polygon.rs");
}