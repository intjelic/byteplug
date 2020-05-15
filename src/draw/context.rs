// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, May 2020

use winit::dpi::PhysicalSize;
use glutin::{ContextBuilder, Context, NotCurrent, GlRequest, GlProfile, Api};
use crate::draw::gl;
use crate::application::get_or_create_event_loop;

static mut CONTEXT: Option<Context<NotCurrent>> = None;

fn ensure_context() {
    unsafe {
        // todo: there must be a more elegant way to 'initialize only if the content is None
        match &CONTEXT {
            Some(_context) => (),
            None => {
                let event_loop = get_or_create_event_loop();
                let context = ContextBuilder::new()
                    .with_gl(GlRequest::Specific(Api::OpenGlEs, (3, 2)))
                    .with_gl_profile(GlProfile::Core)
                    .build_headless(&event_loop, PhysicalSize::new(1, 1))
                    .unwrap();

                CONTEXT = Some(context);
            }
        }
    }
}

/// Get or create the shared OpenGL context
///
/// The **get_or_create_context() function** is not documented yet. Pull requests are welcome.
///
/// **Notes**
///
/// - This function is public because it's used internally by the other module. However, it should
///   not be used by the user.
/// - In terms of design, a hidden and shared OpenGL context allows a 'context notion' free API.
///   Without it, the user would have to create a context object somehow and pass it around.
/// - Note that this shared context also allow to reuse OpenGL object to draw on the framebuffers
///   belonging to other OpenGL contexts.
///
///
pub fn get_or_create_context() -> &'static mut Context<NotCurrent> {
    ensure_context();
    unsafe {
        CONTEXT.as_mut().unwrap()
    }
}

/// Make the shared OpenGL context current
///
/// The **make_context_current() function** is not documented yet. Pull requests are welcome.
///
pub fn make_context_current() {
    ensure_context();

    unsafe {
        let mut context = CONTEXT.take().unwrap();
        let current_context = context.make_current().unwrap();
        CONTEXT = Some(current_context.treat_as_not_current());
    };
}
