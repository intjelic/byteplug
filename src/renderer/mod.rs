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
pub mod gl {
    //! The OpenGL module
    //!
    //! This module exposes the OpenGL ES 3.3 that the renderer module is
    //! using to perform rendering.
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod glyph;
mod font;

mod context;

mod view;
mod surface;

mod shader;
mod texture;

mod text;

pub use glyph::Glyph;
pub use font::Font;

pub use context::Context;

pub use view::View;
pub use surface::Surface;

pub use shader::Shader;
pub use texture::Texture;

pub use text::Text;

pub mod shapes;