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
mod color;
mod gradient;

mod view;
mod surface;

mod renderable;
mod renderer;

pub mod shapes;

pub use color::Color;
pub use gradient::Gradient;

pub use view::View;
pub use surface::Surface;

pub use renderable::Renderable;
pub use renderer::Renderer;