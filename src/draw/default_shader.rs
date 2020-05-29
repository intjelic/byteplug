// Copyright (c) 2020 - BytePlug
//
// This source file is part of Distilled Multimedia Library which is released
// under the MIT license. Please refer to the LICENSE file that can be found
// at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, May 2020

use crate::draw::context::get_or_create_context;
use crate::draw::shader::Shader;

static mut DEFAULT_SHADER: Option<Shader> = None;

const VERTEX_SHADER_SRC: &'static [u8] = b"
#version 300 es
precision mediump float;

layout(location = 0) in vec2 vertex_position;
layout(location = 1) in vec3 vertex_color;

out vec3 fragment_color;

void main() {
    gl_Position = vec4(vertex_position, 0.0, 1.0);
    fragment_color = vertex_color;
}
\0";

const FRAGMENT_SHADER_SRC: &'static [u8] = b"
#version 300 es
precision mediump float;

in vec3 fragment_color;
out vec4 out_color;

void main() {
    out_color = vec4(fragment_color, 1.0);
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
pub fn get_or_create_default_shader() -> &'static mut Shader {
    ensure_default_shader();
    unsafe {
        DEFAULT_SHADER.as_mut().unwrap()
    }
}
