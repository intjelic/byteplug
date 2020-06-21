// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
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
//! The goal is to provide an 'easy and quick to use' interface to the powerful
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
    //! This module exposes OpenGL ES 3.2 API that the draw module uses to perform the rendering.
    //!
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[macro_export]
macro_rules! gl_check {
    ($s:stmt) => {
        $s
        if cfg!(debug_assertions) {
            let err = gl::GetError();
            match err {
                gl::NO_ERROR => (),
                _ => {
                    let err_str = match err {
                        gl::INVALID_ENUM => "GL_INVALID_ENUM",
                        gl::INVALID_VALUE => "GL_INVALID_VALUE",
                        gl::INVALID_OPERATION => "GL_INVALID_OPERATION",
                        gl::INVALID_FRAMEBUFFER_OPERATION => "GL_INVALID_FRAMEBUFFER_OPERATION",
                        gl::OUT_OF_MEMORY => "GL_OUT_OF_MEMORY",
                        gl::STACK_UNDERFLOW => "GL_STACK_UNDERFLOW",
                        gl::STACK_OVERFLOW => "GL_STACK_OVERFLOW",
                        _ => "unknown error"
                    };
                    println!("{}:{} - {} caused {}",
                            file!(),
                            line!(),
                            stringify!($s),
                            err_str);
                }
            }
        }
    }
}

pub(crate) use context::get_or_create_context;
pub(crate) use context::make_context_current;
pub(crate) use default_shader::get_or_create_default_shader;

mod context;

mod options;
mod surface;
mod view;

mod uniform;
mod shader;
mod default_shader;
mod texture;

mod primitive;
mod usage;
mod vertex;
mod vertex_array;

mod shape;

mod glyph;
mod font;
mod text;

pub use options::Options;
pub use surface::Surface;
pub use view::View;

pub use uniform::Uniform;
pub use shader::Shader;
pub use texture::Texture;

pub use primitive::Primitive;
pub use usage::Usage;
pub use vertex::Vertex;
pub use vertex_array::VertexArray;

pub use shape::Shape;

pub use glyph::Glyph;
pub use font::Font;
pub use text::Text;
