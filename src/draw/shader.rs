// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, May 2020

use std::ffi::CString;
use std::string::String;
use crate::draw::gl;
use crate::draw::Uniform;

/// A drawing program stored on the graphics card.
///
/// The **Shader struct** is not documented yet. Pull requests are welcome.
///
/// **Implementation notes**
///
/// - Very primitive implementation of the shader class; will obviously change a lot.
/// - Boolean uniform setter is missing.
/// - Implement the destructor (free shaders and the program as described in the OpenGL docs)
///
#[allow(dead_code)]
pub struct Shader {
    vertex_shader: gl::types::GLuint,
    fragment_shader: gl::types::GLuint,
    program: gl::types::GLuint
}

fn compile_shader(source: &[u8], type_: u32) -> Result<u32, String> {
    unsafe {
        // Create the OpenGL shader object and attach its source code
        gl_check!(let shader = gl::CreateShader(type_));
        gl_check!(gl::ShaderSource(
            shader,
            1,
            [source.as_ptr() as *const _].as_ptr(),
            std::ptr::null(),
        ));

        // Compile the OpenGL shader and check for errors
        gl_check!(gl::CompileShader(shader));

        let mut success = 0;
        gl_check!(gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success));

        if success == gl::FALSE as _ {
            let mut length: i32 = 0;
            let mut info_log: [u8; 1024] = [0; 1024];
            gl_check!(gl::GetShaderInfoLog(shader,
                                 1024,
                                 &mut length,
                                 info_log.as_mut_ptr() as *mut _
            ));
            assert_ne!(length, 0);

            // Make a UTF-8 Rust string out of the ASCII C 'info log' string (note that ASCII is
            // valid UTF-8)
            Err(String::from_utf8_unchecked(info_log.split_at(length as usize).0.to_vec()))
        }
        else {
            Ok(shader)
        }
    }
}

fn make_program(shaders: Vec<u32>) -> Result<u32, String> {
    unsafe {
        gl_check!(let program = gl::CreateProgram());

        // Technically, it should check validity of the shaders (can't attach multiple shaders of
        // the same type).
        for shader in shaders.iter() {
            gl_check!(gl::AttachShader(program, *shader));
        }

        gl_check!(gl::LinkProgram(program));

        // Check for linking status
        let mut success = 0;
        gl_check!(gl::GetProgramiv(program, gl::LINK_STATUS, &mut success));

        if success == gl::FALSE as _ {
            let mut length: i32 = 0;
            let mut info_log: [u8; 1024] = [0; 1024];

            gl_check!(gl::GetProgramInfoLog(program,
                                            1024,
                                            &mut length,
                                            info_log.as_mut_ptr() as *mut _
            ));
            assert_ne!(length, 0);

            // Make a UTF-8 Rust string out of the ASCII C 'info log' string (note that ASCII is
            // valid UTF-8)
            Err(String::from_utf8_unchecked(info_log.split_at(length as usize).0.to_vec()))
        }
        else {
            Ok(program)
        }
    }
}

impl Shader {
    /// Brief description
    ///
    /// The **new() method** is not documented yet. Pull requests are welcome.
    ///
    pub fn new(vertex_shader_src: &[u8], fragment_shader_src: &[u8]) -> Shader {
        let vertex_shader = compile_shader(vertex_shader_src, gl::VERTEX_SHADER)
            .expect("Failed to compile the vertex shader");

        let fragment_shader = compile_shader(fragment_shader_src, gl::FRAGMENT_SHADER)
            .expect("Failed to compile the fragment shader");

        let program = make_program(vec![vertex_shader, fragment_shader])
            .expect("Failed to make the shader program");

        Shader {
            vertex_shader: vertex_shader,
            fragment_shader: fragment_shader,
            program: program
        }
    }

    /// Brief description
    ///
    /// The **set_uniform() method** is not documented yet. Pull requests are welcome.
    ///
    pub fn set_uniform(&mut self, name: &str, uniform: Uniform) {
        self.bind();

        let location = unsafe {
            gl_check!(let location = gl::GetUniformLocation(self.program, CString::new(name).unwrap().into_raw()));

            location
        };

        match uniform {
            Uniform::Integer(value) => {
                unsafe {
                    gl_check!(gl::Uniform1i(location, value));
                }
            },
            Uniform::UnsignedInteger(value) => {
                unsafe {
                    gl_check!(gl::Uniform1ui(location, value));
                }
            },
            Uniform::Float(value) => {
                unsafe {
                    gl_check!(gl::Uniform1f(location, value));
                }
            },
            Uniform::Vector2(x, y) => {
                unsafe {
                    gl_check!(gl::Uniform2f(location, x, y));
                }
            },
            Uniform::Vector3(x, y, z) => {
                unsafe {
                    gl_check!(gl::Uniform3f(location, x, y, z));
                }
            },
            Uniform::Matrix2(value) => {
                unsafe {
                    gl_check!(gl::UniformMatrix2fv(location, 1, gl::FALSE, value.as_ptr() as _));
                }
            },
            Uniform::Matrix3(value) => {
                unsafe {
                    gl_check!(gl::UniformMatrix3fv(location, 1, gl::FALSE, value.as_ptr() as _));
                }
            },
            Uniform::Matrix4(value) => {
                unsafe {
                    gl_check!(gl::UniformMatrix4fv(location, 1, gl::FALSE, value.as_ptr() as _));
                }
            }
        }
    }

    /// Brief description
    ///
    /// The **bind() method** is not documented yet. Pull requests are welcome.
    ///
    pub fn bind(&self) {
        unsafe {
            gl_check!(gl::UseProgram(self.program));
        }
    }
}
