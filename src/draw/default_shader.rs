// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, May 2020

use crate::draw::context::{get_or_create_context, make_context_current};
use crate::draw::shader::Shader;

static mut DEFAULT_SHADER: Option<Shader> = None;

const VERTEX_SHADER_SRC: &'static [u8] = b"
#version 300 es
precision mediump float;

layout(location = 0) in vec2 vertex_position;
layout(location = 1) in vec3 vertex_color;
layout(location = 2) in vec2 vertex_texture;

out vec3 fragment_color;
out vec2 fragment_texture;

void main() {
    gl_Position = vec4(vertex_position, 0.0, 1.0);

    fragment_color = vertex_color;
    fragment_texture = vertex_texture;
}
\0";

const FRAGMENT_SHADER_SRC: &'static [u8] = b"
#version 300 es
precision mediump float;

in vec3 fragment_color;
in vec2 fragment_texture;
out vec4 out_color;

uniform sampler2D current_texture;

void main() {
    out_color = texture(current_texture, fragment_texture);
}
\0";

fn ensure_default_shader() {
    unsafe {
        // todo: there must be a more elegant way to 'initialize only if the content is None
        match &DEFAULT_SHADER {
            Some(_) => (),
            None => {
                // Make sure context is created
                let _context = get_or_create_context();
                make_context_current();

                let default_shader = Shader::new(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC);
                DEFAULT_SHADER = Some(default_shader);
            }
        }
    }
}

/// Get or create the default OpenGL shader program.
///
/// The **get_or_create_default_shader() function** is not documented yet. Pull requests are
/// welcome.
///
pub(crate) fn get_or_create_default_shader() -> &'static mut Shader {
    ensure_default_shader();
    unsafe {
        DEFAULT_SHADER.as_mut().unwrap()
    }
}
