// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use winit::dpi::PhysicalSize;
use glutin::{
    GlProfile, GlRequest, Api,
    ContextBuilder,
    Context, RawContext,
    NotCurrent
};
use crate::geometry::Size;
use crate::draw::context::get_or_create_context;
use crate::draw::{gl, Options};
use crate::application::get_or_create_event_loop;

/// The underlying glutin context type
///
/// The definition of the OpenGL context as per the **glutin** module will be different. In the case
/// of a 'window surface' we will need `RawContext` because it exposes a resize method which is
/// needed to resize the default framebuffer. In the traditional case, it will be a simle `Context`
/// (and we'll resize the regular framebuffer with OpenGL calls).
///
enum UnderlyingContext {
    NoWindow(Context<NotCurrent>),
    WithWindow(RawContext<NotCurrent>)
}

/// A drawing area of a fixed size
///
/// The **Surface struct** is not documented yet. Pull requests are welcome.
///
/// A surface is a drawing area of a fixed size which is hardware-accelerated. To draw, on it, you
/// define points which are then linked
///
/// **Notes**
///
/// - All the surfaces create one OpenGL context, even if it's not always necessary, and it's a core
///   design decision that should be understood (it roots from the fact that a surface can also be
///   used to represent the surface of a window).
/// - A surface represents a drawing area of a fixed size which is either virtual (not directly
///   visible) or real (directly visible, mapped to the surface of a window).
/// - In both cases, the underlying OpenGL concept is a 'framebuffer', but in the case of a window,
///   it's the 'default fraembuffer' of a OpenGL context and in the case of a non-window surface,
///   it's a framebuffer that must be created and bound later. It's important to notice that to
///   create the framebuffer for a window, an OpenGL context MUST be created, whereas the non-window
///   framebuffers can be created indefinitvely from a single OpenGL context.
/// - It's the reason why having one OpenGL context per surface simplifies enormouesly the
///   implementation as now, we only have to think in terms of making the OpenGL context associated
///   to the surface current; in the case of a window, no framebuffer is bound, OpenGL will draw
///   on the default framebuffer, in the case of a non-window surface, the single framebuffer is
///   always bound, OpenGL will draw on the bound framebuffer.
/// - As an example, resizing the surface will be about either resizing the default framebuffer in
///   the case of a window, or the generated framebuffer in the case of non-window.
/// - By the terms used by `glutin`, 'raw' contexts are created for windows (using their handles)
///   and headless contexts are created for the non-window surfaces. Note that headless winodws are
///   created with size (1, 1) for their default framebuffer but it doesn't matter because what
///   matters in their case is the framebuffer that is created later.
/// - The notion of surface size overlaps with the notion of window size. It's the user's
///   responsability to resize the surface according to the window size.
pub struct Surface {
    context: Option<UnderlyingContext>, // shouldn't be a Option, but the make_current() methods consume themselves
    render_buffer: u32, // not used in the case of a window surface
    frame_buffer: u32,  // not used in the case of a window surface
    size: Size
}

impl Surface {
    /// Brief description
    ///
    /// The **new() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn new(size: Size, options: Options) -> Surface {

        let event_loop = get_or_create_event_loop();
        let shared_context = get_or_create_context();

        let context_builder = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGlEs, (3, 2)))
            .with_gl_profile(GlProfile::Core)
            .with_srgb(false)
            .with_multisampling(0)
            .with_shared_lists(shared_context);

        let size_one = PhysicalSize::new(1, 1);
        let context = context_builder.build_headless(&event_loop, size_one).unwrap();

        let mut render_buffer = 0;
        let mut frame_buffer = 0;
        unsafe {
            gl::GenRenderbuffers(1, &mut render_buffer);
            gl::BindRenderbuffer(gl::RENDERBUFFER, render_buffer);
            gl::RenderbufferStorage(
                gl::RENDERBUFFER,
                gl::RGB8,
                size.width as _,
                size.height as _,
            );
            gl::GenFramebuffers(1, &mut frame_buffer);
            gl::BindFramebuffer(gl::FRAMEBUFFER, frame_buffer);
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::RENDERBUFFER,
                render_buffer,
            );

            gl::Viewport(0, 0, size.width as _, size.height as _);
        }

        Surface {
            context: Some(UnderlyingContext::NoWindow(context)),
            render_buffer: render_buffer,
            frame_buffer: frame_buffer,
            size: size
        }
    }

    /// Brief description
    ///
    /// The **from_window() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn from_window(context: RawContext<NotCurrent>, size: Size) -> Surface {
        unsafe {
            gl_check!(gl::Viewport(0, 0, size.width as _, size.height as _));
        }

        Surface {
            context: Some(UnderlyingContext::WithWindow(context)),
            render_buffer: 0, // not used
            frame_buffer: 0,  // not used
            size: size
        }
    }

    /// Brief description
    ///
    /// The **resize() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn resize(&mut self, size: Size) {
        // The code is a little bit tricky because make_current() methods of the context consume
        // the context, but we only have a reference to self, and thus a reference to the context
        // (we cannot take ownership, even temporarily).

        let mut context = self.context.take().unwrap();
        context = match context {
            UnderlyingContext::NoWindow(mut underlying_context) => {
                // A regular (non-window) surface; resize the framebuffer using OpenGL calls after
                // we make the OpenGL context current.
                underlying_context = unsafe {
                    let current_context = underlying_context.make_current().unwrap();

                    // todo: destory the generated buffer and re-create it

                    current_context.treat_as_not_current()
                };

                UnderlyingContext::NoWindow(underlying_context)
            },
            UnderlyingContext::WithWindow(mut underlying_context) => {
                // A window surface; resize the default framebuffer of the OpenGL context using the
                // context instance itself.
                underlying_context = unsafe {
                    let current_context = underlying_context.make_current().unwrap();
                    current_context.resize(winit::dpi::PhysicalSize::new(size.width as _, size.height as _));
                    unsafe {
                        gl_check!(gl::Viewport(0, 0, size.width as _, size.height as _));
                    }
                    current_context.treat_as_not_current()
                };

                UnderlyingContext::WithWindow(underlying_context)
            }
        };

        self.context = Some(context);
    }

    /// Brief description
    ///
    /// The **erase() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn erase(&mut self) {
        // Todo: make the context current
        let color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        unsafe {
            gl::ClearColor(color[0], color[1], color[2], color[3]);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    /// Brief description
    ///
    /// The **swap() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn swap(&mut self) {
        // The code is more complicated than it could. We have to take ownership of the underlying
        // context because the making it current method consumes itself.
        let mut context = self.context.take().unwrap();
        context = match context {
            UnderlyingContext::NoWindow(underlying_context) =>
                // Nothing to do in the case of a non-window surface, there's no double buffering.
                UnderlyingContext::NoWindow(underlying_context),
            UnderlyingContext::WithWindow(mut underlying_context) => {
                // In the case of a window surface, we have to call the swap_buffer() from the
                // OpenGL context instance.
                underlying_context = unsafe {
                    let current_context = underlying_context.make_current().unwrap();
                    current_context.swap_buffers().unwrap();

                    current_context.treat_as_not_current()
                };

                UnderlyingContext::WithWindow(underlying_context)
            }
        };

        self.context = Some(context);
    }

    /// Brief description
    ///
    /// The **activate() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn activate(&mut self) {
        // Again, the code is more complicated than it could. Not only we have to take ownership
        // of the underlying context again, but we also have to deal with two definitions of the
        // context (RawContext when it's a window surface, and Context when it's a non-window
        // surface). Note that we could have borrowed the inner OpenGL context from RawContext and
        // have a common call  to make the context current, but given make_current() consumes itself
        // and we only have a reference, we cannot.
        let mut context = self.context.take().unwrap();
        context = match context {
            UnderlyingContext::NoWindow(mut underlying_context) => {
                underlying_context = unsafe {
                    underlying_context.make_current().unwrap().treat_as_not_current()
                };

                UnderlyingContext::NoWindow(underlying_context)
            },
            UnderlyingContext::WithWindow(mut underlying_context) => {
                underlying_context = unsafe {
                    underlying_context.make_current().unwrap().treat_as_not_current()
                };

                UnderlyingContext::WithWindow(underlying_context)
            }
        };

        self.context = Some(context);
    }
}
