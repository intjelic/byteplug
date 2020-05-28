// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, May 2020

use crate::geometry::{Position, Size};
use crate::geometry::Box;
use crate::draw::gl;
use crate::draw::context::{get_or_create_context, make_context_current};
use crate::draw::{Vertex, Primitive, Usage};
use crate::draw::Surface;
use crate::draw::default_shader::get_or_create_default_shader;

const VERTEX_POSITION: u32 = 0;
const VERTEX_COLOR: u32 = 1;
const VERTEX_TEXTURE: u32 = 2;

fn from_usage(usage: Usage) -> gl::types::GLenum {
    match usage {
        Usage::Static  => gl::STATIC_DRAW,
        Usage::Dynamic => gl::DYNAMIC_DRAW,
        Usage::Stream  => gl::STREAM_DRAW
    }
}

fn to_usage(usage: gl::types::GLenum) -> Usage {
    match usage {
        gl::STATIC_DRAW  => Usage::Static,
        gl::DYNAMIC_DRAW => Usage::Dynamic,
        gl::STREAM_DRAW  => Usage::Stream,
        _ => panic!("Unexpected usage specifier returned by the graphics driver (buggy driver ?)")
    }
}

fn from_primitive(primitive: Primitive) -> gl::types::GLenum {
    match primitive {
        Primitive::Points          => gl::POINTS,
        Primitive::Lines           => gl::LINES,
        Primitive::LineStrips      => gl::LINE_STRIP,
        Primitive::Triangles       => gl::TRIANGLES,
        Primitive::TriangleStrips  => gl::TRIANGLE_STRIP,
        Primitive::TriangleFans    => gl::TRIANGLE_FAN
    }
}

/// An array of vertices stored on the graphics cards.
///
/// A vertex array is a set of vertices stored directly stored on the graphics memory, associated
/// with a drawing primitive. Together, they tell the graphics card how to draw an object on a
/// surface. Note that the graphics card understand only vertices and this is your only mean for
/// drawing. Also, because the data lives on the graphics memory, and not on the system memory, you
/// must also use it adequately.
///
/// If you draw simple shapes, images, and texts, you normally don't use vertex arrays directly;
/// instead, you use the high-level drawable primitives that this module is providing (which
/// themselves use a vertex array internally). However, if you have very specific drawing needs,
/// using vertex arrays allows you to render on a surface in all sorts of way. It also allows you to
/// store vertices more efficiently than if you were to use a combined set of the high-level
/// drawing primitives. For instance, an efficient particles or tilling system will use vertex
/// arrays directly.
///
/// To update the set of vertices, use the `update_vertices()` method which will dynamically
/// increase or decrease the size of the array. The drawing primitive specifies how to use the
/// vertices to form shapes on the surface, and the usage specifier gives the graphics card a hint
/// about memory usage (for instance, if you hardly ever change the vertices, giving `Static` as
/// hint will allow the graphics card to perform better). The default primitive is `Points` and the
/// default usage is `Stream`.
///
/// **Implementation notes**
///
/// - A good read to understand the implementation is the 'Buffer Object' page of OpenGL
///   wiki: https://www.khronos.org/opengl/wiki/Buffer_Object
/// - I didn't spend much time on designing this interface; it may evolve to be more consistent with
///   the overall framework interface.
/// - It's difficult to implement a rich and flexible interface like for `Vec<T>` because the data
///   are stored on the graphics memory and it's costly. Plus, it's best to keep it simple; fancy
///   operations can be done using regular vectors on system memory, and then the vertex array
///   updated once with the `update_vertices()` method.
/// - From what I understood, there may be a way to map the graphics memory to system memory and
///   drastically change the implementation (and interface!). Instead of providing a 'copy API'
///   where vertices are copied around (the vertices() method returns a copy, the update_vertices()
///   method implementation does copy), we could implement a vector-like API returning sliced and
///   the standard 'array operations' because the internal `Vec<Vertex>` would directly be acting on
///   the mapped memory. There would be a need to 'flush' the mapped memory so OpenGL knows that it
///   has to synchronize it with the graphics memory; it could be either done with an update()
///   method or systematically right before it's used for drawing. The advantage of this
///   implementation is that the user wouldn't have to make a separate copy of the vertices, make
///   transformations on it, then re-update the vertex array. This is to be investigated.
/// - Post-implementation note: The idea of above seems to be totally applicable. In fact, it would
///   be much better and the implementation of the shapes would benefit from them (since they're
///   using vertex arrays internally).
/// - I could not find a way to change the 'usage' of the OpenGL buffer objects; it seems like the
///   only way is to re-create it. Therefore, changing the usage property is marked as slow
///   operation, and to be avoided after the vertex array contains vertices.
/// - Regarding the implementation, it's important to understand that a vertex array with no data
///   doesn't have an OpenGL object buffer created just yet. One consequence of that is that we must
///   keep an 'extra' usage field so when the user is updating the vertex array with actual data, we
///   know which usage to set (usage can be set before vertices are updated). When the user queries
///   the usage, we rely on the actual usage (as returned by OpenGL) unless there's no buffer yet in
///   which case we really on our local one.
/// - Post-implementation note: I removed the with_size() constructor and the resize() method because
///   they make the implementation more complicated. To resize the array, the user use the
///   update_vertices() method for now. The user will have more control over the memory after it's
///   updated to use the 'mapped memory' technique (see other notes).
/// - Post-implementation note: I drastically simplified the implementation of update_vertices()
///   because the whole implementation will likely be reworked to used the 'mapped object buffer'
///   technique. Thus, for now, it simply re-creates an entire object buffer, discarding the
///   existing one.
///
pub struct VertexArray {
    buffer: u32,
    primitive: Primitive,
    usage: Usage, // see notes
    bounds: Box<f32> // will be removed after mapped buffer technique is used
}

impl VertexArray {
    /// Constructs an empty vertex array.
    ///
    /// This function is the default constructor. It creates a vertex array with no vertices, using
    /// the points primitive and the stream usage specifier.
    ///
    pub fn new() -> VertexArray {
        // Make sure the shared OpenGL context is created (it doesn't matter which context is
        // active; the OpenGL object buffer will be shared).
        let _context = get_or_create_context();
        make_context_current();

        VertexArray {
            buffer: 0,
            primitive: Primitive::Points,
            usage: Usage::Stream,
            bounds: Box::default()
        }
    }

    /// Constructs a vertex array from a set of vertices and a primitive.
    ///
    /// This function is a constructor that constructs a vertex array from a list of vertices, their
    /// drawing primitive and an usage specifier.
    ///
    pub fn with_vertices(vertices: &Vec<Vertex>, primitive: Primitive, usage: Usage) -> VertexArray {

        let mut vertex_array = Self::new();

        vertex_array.set_primitive(primitive);
        vertex_array.set_usage(usage); // This one should be done before updating the vertices (see implementation detail)
        vertex_array.update_vertices(vertices);

        vertex_array
    }

    /// Returns the number of vertices in the array.
    ///
    /// This function returns the number of vertices in the array.
    ///
    pub fn size(&self) -> usize {

        // The vertex array may not have an OpenGL object buffer yet, in this case, the size
        // obviously is zero.
        if self.buffer == 0 {
            return 0
        }

        // Make the OpenGL object buffer current (so GetBufferParameteriv() operates on it).
        self.bind();
        unsafe {
            // Ask OpenGL the size of the OpenGL object buffer in bytes (size isn't expected to be
            // zero otherwise there wouldn't be any OpenGL object buffer).
            let mut buffer_size: i32 = 0;
            gl_check!(gl::GetBufferParameteriv(
                gl::ARRAY_BUFFER,
                gl::BUFFER_SIZE,
                &mut buffer_size
            ));
            assert_ne!(buffer_size, 0);

            // Convert the size in bytes to size 'in Vertex' (todo; should it panic if the size
            // isn't perfectly dividable ?).
            let size = buffer_size / std::mem::size_of::<Vertex>() as i32;
            size as usize
        }
    }

    /// Returns a copy of the vertices in the array.
    ///
    /// This function returns a copy of the vertices in the array. They're copied from graphics
    /// memory to system memory, which is a heavy operation, therefore you should not use it
    /// adequately.
    ///
    pub fn vertices(&self) -> Vec<Vertex> {

        // The vertex array may not have an OpenGL object buffer yet, in this case, it's an empty
        // vector.
        if self.buffer == 0 {
            return Vec::<Vertex>::new()
        }

        // We could have used GetBufferSubData() but it's not available in OpenGL ES 3.2, therefore,
        // we must map the data (on graphics memory) to the system memory with MapBufferRange()
        // then copy it into a `Vec<Vertex>`.
        let size = self.size();

        // Make the OpenGL object buffer current (so MapBufferRange() operates on it).
        self.bind();
        let vertices = unsafe {
            let length = size * std::mem::size_of::<Vertex>();
            let pointer = gl::MapBufferRange(
                gl::ARRAY_BUFFER,
                0,
                length as _,
                gl::MAP_READ_BIT
            );

            // The strategy is to make a `Vec<Vertex>` from the mapped data (which we don't own), so
            // we can easily make a copy of it. We manually ask not to drop the data as we don't own
            // it.
            let mapped_vertices = Vec::<Vertex>::from_raw_parts(
                pointer as _,
                size,
                size
            );
            let vertices = mapped_vertices.clone();
            std::mem::ManuallyDrop::new(mapped_vertices);

            vertices
        };

        vertices
    }

    /// Update the vertices in the array.
    ///
    /// This function updates the vertices in the array, discarding the previous vertices and
    /// possibly resizing the array. It's a heavy operation that consists of copying system memory
    /// to graphics memory, and therefore should be used adequately.
    ///
    pub fn update_vertices(&mut self, vertices: &Vec<Vertex>) {
        // This function was heavily simplified because the entire implementation will be changed
        // later to use the 'mapped memory' technique. Instead of intelligently checking and
        // possibly resizing the buffer, it just destroys it and re-creates one with the new
        // vertices.

        // Delete the existing OpenGL object buffer if there is one.
        if self.buffer != 0 {
            self.delete_buffer();
        }

        // If the list of vertices provided by the user is empty, there actually is nothing to do
        // other than resetting the bounding box.
        if vertices.is_empty() {
            self.bounds = Box::default();

            return
        }

        // If the list is not empty, we generate the OpenGL object buffer, make it current, and
        // upload the vertices to it.
        self.generate_buffer();
        self.bind();
        unsafe {
            gl_check!(gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const _,
                from_usage(self.usage),
            ));
        };

        // Re-compute the bounding box.
        self.bounds = Self::compute_bounds(&vertices);
    }

    pub fn bounds(&self) -> Box<f32> {
        self.bounds
    }

    /// Returns the drawing primitive.
    ///
    /// This function returns the drawing primitive used to render the vertices.
    ///
    pub fn primitive(&self) -> Primitive {

        self.primitive
    }

    /// Changes the drawing primitive.
    ///
    /// This function changes the drawing primitive used to render the vertices.
    ///
    pub fn set_primitive(&mut self, primitive: Primitive) {

        self.primitive = primitive;
    }

    /// Returns the usage specifier.
    ///
    /// This function returns the usage specifier.
    ///
    pub fn usage(&self) -> Usage {

        // The vertex array may not have an OpenGL object buffer yet, in this case, the usage is
        // the local one.
        if self.buffer == 0 {
            return self.usage
        }

        // Make the OpenGL object buffer current (so GetBufferParameteriv() operates on it).
        self.bind();
        unsafe {
            // Ask OpenGL the usage of the OpenGL object buffer (it's expected to be the same as the
            // local one otherwise it's an implementation error).
            let mut buffer_usage: i32 = 0;
            gl_check!(gl::GetBufferParameteriv(
                gl::ARRAY_BUFFER,
                gl::BUFFER_USAGE,
                &mut buffer_usage
            ));
            assert_eq!(to_usage(buffer_usage as _), self.usage);

            to_usage(buffer_usage as _)
        }
    }

    /// Changes the usage specifier.
    ///
    /// This function changes the usage specifier. Note that due to a graphics driver limitation,
    /// it's an expensive operation because the vertices need to be duplicated in graphics memory.
    ///
    pub fn set_usage(&mut self, usage: Usage) {
        // There may be a better way to implement this function. There doesn't seem to be a way to
        // change the usage specifier without re-creating the OpenGL object buffer. That being said,
        // the current implementation makes it even worst as it copies vertices back to system
        // memory to re-copy them back to graphics memory. The existing graphics memory could simply
        // be copied into a new OpenGL object buffer that has the new usage specifier before the old
        // one is deleted (that would involve no transfer system/graphics memory).
        self.usage = usage;
        self.update_vertices(&self.vertices());
    }

    /// Draws the vertex array on a surface.
    ///
    /// This functions draws the vertex array on a surface according to its drawing primitive.
    ///
    pub fn draw(&self, surface: &mut Surface) {

        // To draw on the surface, we must make its underlying OpenGL context (and thus associated
        // framebuffer) current. This is so the DrawArrays() function operates on it.
        surface.activate();

        // The vertex array is using the default shader program; make it current.
        let default_shader = get_or_create_default_shader();
        default_shader.bind();

        // Make the OpenGL object buffer current (so VertexAttribPointer() and
        // EnableVertexAttribArray() operate on it).
        self.bind();
        unsafe {
            // Draw the vertices by using the correct OpenGL drawing primitive.
            gl_check!(gl::VertexAttribPointer(
                VERTEX_POSITION,
                2,
                gl::FLOAT,
                0,
                std::mem::size_of::<Vertex>() as gl::types::GLsizei,
                std::ptr::null(),
            ));
            gl_check!(gl::VertexAttribPointer(
                VERTEX_COLOR,
                3,
                gl::FLOAT,
                0,
                std::mem::size_of::<Vertex>() as gl::types::GLsizei,
                (2 * std::mem::size_of::<f32>()) as *const () as *const _,
            ));
            gl_check!(gl::VertexAttribPointer(
                VERTEX_TEXTURE,
                2,
                gl::FLOAT,
                0,
                std::mem::size_of::<Vertex>() as gl::types::GLsizei,
                (6 * std::mem::size_of::<f32>()) as *const () as *const _,
            ));
            gl_check!(gl::EnableVertexAttribArray(VERTEX_POSITION));
            gl_check!(gl::EnableVertexAttribArray(VERTEX_COLOR));
            gl_check!(gl::EnableVertexAttribArray(VERTEX_TEXTURE));

            gl_check!(gl::DrawArrays(from_primitive(self.primitive), 0, self.size() as _));
        }
    }

    /// Binds the vertex array.
    ///
    /// This function binds the underlying OpenGL object buffer. It must be used only if you mix the
    /// vertex array with your own OpenGL code.
    ///
    pub fn bind(&self) {

        // The vertex array may not have an OpenGL object buffer yet, in this case, there's nothing
        // to do.
        if self.buffer == 0 {
            return
        }

        unsafe {
            gl_check!(gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer));
        }
    }

    fn generate_buffer(&mut self) {
        assert_eq!(self.buffer, 0);
        let buffer = unsafe {
            let mut buffer: u32 = 0;
            gl_check!(gl::GenBuffers(1, &mut buffer));

            buffer
        };
        assert_ne!(buffer, 0);

        self.buffer = buffer;
    }

    fn delete_buffer(&mut self) {
        assert_ne!(self.buffer, 0);
        unsafe {
            gl::DeleteBuffers(1, &mut self.buffer)
        }
        self.buffer = 0;
    }

    fn compute_bounds(vertices: &Vec<Vertex>) -> Box<f32> {
        // There could be a smarter way to implement this and merge it with the code of
        // `compute_bounds()` but for now it's unnecessarily complicated.
        if vertices.is_empty() {
            return Box::default()
        }

        let mut left   = vertices[0].x;
        let mut top    = vertices[0].y;
        let mut right  = vertices[0].x;
        let mut bottom = vertices[0].y;

        for vertex in vertices.iter() {
            if vertex.x < left {
                left = vertex.x;
            }
            else if vertex.x > right {
                right = vertex.x;
            }

            if vertex.y < top {
                top = vertex.y;
            }
            else if vertex.y > bottom {
                bottom = vertex.y;
            }
        };

        Box::new(Position::new(left, top), Size::new(right - left, bottom - top))
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        // If the vertex array never had any vertices, no OpenGL object buffer was created, in this
        // case there's nothing to do.
        if self.buffer == 0 {
            return
        }

        unsafe {
            gl::DeleteBuffers(1, &mut self.buffer)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertex_array_new() {
        let vertex_array = VertexArray::new();
        assert_eq!(vertex_array.primitive(), Primitive::Points);
        assert_eq!(vertex_array.usage(), Usage::Stream);
        assert_eq!(vertex_array.vertices().len(), 0);
        assert_eq!(vertex_array.size(), 0);
    }

    #[test]
    fn vertex_array_with_vertices() {
        let vertices = vec![
            Vertex { x: -0.5, y: -0.5, r: 1.0, g: 0.0, b: 0.0, a: 0.0, u: 0.0, v: 0.0 },
            Vertex { x:  0.0, y:  0.5, r: 0.0, g: 1.0, b: 0.0, a: 0.0, u: 0.0, v: 0.0 },
            Vertex { x:  0.5, y: -0.5, r: 0.0, g: 0.0, b: 1.0, a: 0.0, u: 0.0, v: 0.0 },
        ];

        let vertex_array = VertexArray::with_vertices(&vertices, Primitive::Triangles, Usage::Static);
        assert_eq!(vertex_array.primitive(), Primitive::Triangles);
        assert_eq!(vertex_array.usage(), Usage::Static);
        assert_eq!(vertex_array.vertices(), vertices);
        assert_eq!(vertex_array.size(), 3);
    }

    #[test]
    fn vertex_array_primitive() {
        let mut vertex_array = VertexArray::new();
        assert_eq!(vertex_array.primitive(), Primitive::Points);

        vertex_array.set_primitive(Primitive::Lines);
        assert_eq!(vertex_array.primitive(), Primitive::Lines);

        vertex_array.set_primitive(Primitive::Triangles);
        assert_eq!(vertex_array.primitive(), Primitive::Triangles);
    }

    #[test]
    fn vertex_array_usage() {
        // The implementation of set_usage() is a bit complicated because the 'usage' can't be
        // changed without re-creating the OpenGL object buffer. Therefore, the validity of the
        // vertices must be tested as well.
        let mut vertex_array = VertexArray::new();
        assert_eq!(vertex_array.usage(), Usage::Stream);

        vertex_array.set_usage(Usage::Dynamic);
        assert_eq!(vertex_array.usage(), Usage::Dynamic);

        let vertices = vec![
            Vertex { x: -0.5, y: -0.5, r: 1.0, g: 0.0, b: 0.0, a: 0.0, u: 0.0, v: 0.0 },
            Vertex { x:  0.0, y:  0.5, r: 0.0, g: 1.0, b: 0.0, a: 0.0, u: 0.0, v: 0.0 },
            Vertex { x:  0.5, y: -0.5, r: 0.0, g: 0.0, b: 1.0, a: 0.0, u: 0.0, v: 0.0 },
        ];
        vertex_array.update_vertices(&vertices);
        assert_eq!(vertex_array.usage(), Usage::Dynamic);

        vertex_array.set_usage(Usage::Static);
        assert_eq!(vertex_array.usage(), Usage::Static);
        assert_eq!(vertex_array.vertices(), vertices);
    }

    #[test]
    fn vertex_array_vertices() {

        let vertices = vec![
            Vertex { x: -0.5, y: -0.5, r: 1.0, g: 0.0, b: 0.0, a: 0.0, u: 0.0, v: 0.0 },
            Vertex { x:  0.0, y:  0.5, r: 0.0, g: 1.0, b: 0.0, a: 0.0, u: 0.0, v: 0.0 },
            Vertex { x:  0.5, y: -0.5, r: 0.0, g: 0.0, b: 1.0, a: 0.0, u: 0.0, v: 0.0 },
        ];

        let vertex_array = VertexArray::with_vertices(&vertices, Primitive::Triangles, Usage::Static);
        assert_eq!(vertex_array.primitive(), Primitive::Triangles);
        assert_eq!(vertex_array.usage(), Usage::Static);
        assert_eq!(vertex_array.vertices(), vertices);
        assert_eq!(vertex_array.size(), 3);
    }

    #[test]
    fn vertex_array_bounds() {
        // To be implemented.
    }

    #[test]
    fn vertex_array_draw() {
        // To be implemented.
    }

    #[test]
    fn vertex_array_bind() {
        // To be implemented.
    }
}
