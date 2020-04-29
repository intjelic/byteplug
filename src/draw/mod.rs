// Copyright (c) 2020 - BytePlug
//
// This source file is part of Distilled Multimedia Library which is released
// under the MIT license. Please refer to the LICENSE file that can be found
// at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

//! Hardware-accelerated drawing functionalities
//!
//! The **draw module** provides functionalities that are exclusively
//! 2D-oriented in order to draw on the screen or anything that is conceptually
//! a 2D array of pixels such as images.
//!
//! It's an abstraction over **OpenGL ES 3.2** which became the standard to
//! perform hardware-accelerated rendering because it's supported on all major
//! platforms (including the web) and is pretty much the only option. Therefore,
//! you can use this module without worrying about the underlying rendering
//! mechanism; it will pick up what is available, whether it's actually hardware
//! accelerated or software emulated, and work.
//!
//! The goal is to provide a 'quick to use and manage' interface to the powerful
//! rendering arsenal that OpenGL is, while giving the opportunity to more
//! experimented programmer to combine it with direct OpenGL code later, when
//! the need arises.
//!
//! Obviously, very high-demanding programs that require fine-control over the
//! graphical operations doesn't fit the scope of this module and they will be
//! better off without it. That said, not everybody can afford that level of
//! optimization and most of people will find in this module a comfortable spot
//! to work with and extend later.
//!
//! It was heavily inspired from the SFML graphics module. The main difference
//! is that it reflects modern OpenGL programming, some concepts are re-arranged
//! and renamed, and it has a rusty interface.
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
