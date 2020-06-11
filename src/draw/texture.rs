// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use crate::geometry::Size;
use crate::image::{Color, Image};
use crate::draw::context::{get_or_create_context, make_context_current};
use crate::draw::gl;

/// An image stored on the graphics card.
///
/// The **Texture struct** is not documented yet. Pull requests are welcome.
///
/// **Implementation notes**
///
/// - Double-check implementation of the resize() method. According to SFML implementation, there
///   might be some hardware limitation (would need to computer the nearest power of two).
/// - The sRGB and minimap features aren't implemented yet.
/// - For now, the interface of Texture is kept simple and clean. It should be understood as the
///   counter-part of Image. Image provides load/save methods to load the image from a file then
///   save the image to a file. The Texture class also provides load/save methods, but instead it
///   load from an Image and save to an Image.
/// - There's no pixel() and pixel_mut() methods to discourage its use. Updating the pixels of a
///   texture is a slow operation because the pixels are located on the graphics card, it should be
///   done in batch. Instead, the user should do pixel modifications on an image, then re-update the
///   texture with this image.
/// - The set_repeated() implementation is different from SFML, double-check it. It uses the
///   CLAMP_TO_EDGE constant.
/// - Investigate OpenGL sampler objects; they seem to be related to texture and add them features.
///   See what it is and if this item can be improved.
///
pub struct Texture {
    object: gl::types::GLuint
}

impl Texture {
    /// Brief description
    ///
    /// The **new() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn new() -> Texture {
        // Make sure the shared OpenGL context is created (it doesn't matter which context is
        // active; the OpenGL object buffer will be shared).
        let _context = get_or_create_context();
        make_context_current();

        // Create the OpenGL texture object.
        let object = unsafe {
            let mut object = 0;
            gl_check!(gl::GenTextures(1, &mut object));

            object
        };

        let mut texture = Texture {
            object: object
        };

        texture.set_smooth(false);
        texture.set_repeated(false);

        texture
    }

    /// Brief description
    ///
    /// The **with_size() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn with_size(size: Size<i32>, color: Color) -> Texture {
        let mut texture = Texture::new();
        texture.resize(size, color);

        texture
    }

    /// Brief description
    ///
    /// The **with_image() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn with_image(image: &Image) -> Texture {
        let mut texture = Texture::new();
        texture.update_image(image);

        texture
    }

    /// Brief description
    ///
    /// The **open() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn open(filename: &str) -> Texture {
        // To be implemented.
        Texture::new()
    }

    /// Brief description
    ///
    /// The **size() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn size(&self) -> Size<i32> {
        // Make the OpenGL texture object current (so GetTexLevelParameteriv() operates on it).
        self.bind();

        // Ask OpenGL the width and height of the OpenGL texture object.
        let (width, height) = unsafe {
            let mut width: i32 = 0;
            let mut height: i32 = 0;

            gl_check!(gl::GetTexLevelParameteriv(
                gl::TEXTURE_2D,
                0,
                gl::TEXTURE_WIDTH,
                &mut width
            ));

            gl_check!(gl::GetTexLevelParameteriv(
                gl::TEXTURE_2D,
                0,
                gl::TEXTURE_HEIGHT,
                &mut height
            ));

            (width, height)
        };

        Size::new(width, height)
    }

    /// Brief description
    ///
    /// The **resize() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn resize(&mut self, size: Size<i32>, color: Color) {
        // The implementation of this function is a bit complicated; for now, it cleans up the
        // entire area but it should not.
        let size_usize = Size::new(size.width as usize, size.height as usize); // should be removed

        let image = Image::with_size(size_usize, color);
        self.update_image(&image);
    }

    /// Brief description
    ///
    /// The **update_image() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn update_image(&mut self, image: &Image) {
        // Compute height of the image.
        let size = image.size();
        let pixels = image.pixels();

        self.bind();

        unsafe {
            gl_check!(gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as _,
                size.width as _,
                size.height as _,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                pixels.as_ptr() as _
            ));
        }
    }

    /// Brief description
    ///
    /// The **to_image() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn to_image(&self) -> Image {
        // - GetTexPixels is not available, have to create a framebuffer, attach texture and use
        // glReadPixels()
        // - restore previous framebuffer bind ?
        self.bind();

        let size = self.size();

        let capacity = (size.width * size.height) as usize;
        let mut pixels = Vec::<Color>::with_capacity(capacity);
        pixels.resize(capacity, Color::BLACK); // not necessary

        unsafe {
            let mut framebuffer = 0;
            gl_check!(gl::GenFramebuffers(1, &mut framebuffer));
            gl_check!(gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer));

            gl_check!(gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                self.object,
                0
            ));

            gl_check!(gl::ReadPixels(
                0,
                0,
                size.width,
                size.height,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                pixels.as_mut_ptr() as _
            ));

            gl_check!(gl::DeleteFramebuffers(1, &mut framebuffer));
        }

        let size_usize = Size::new(size.width as usize, size.height as usize); // should be removed
        Image::with_pixels(size_usize, pixels).unwrap()
    }

    /// Brief description
    ///
    /// The **is_smooth() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn is_smooth(&self) -> bool {
        // Make the OpenGL texture object current (so GetTexParameteri() operates on it).
        self.bind();

        // Ask OpenGL the smooth property of the OpenGL texture object.
        let smooth = unsafe {
            let mut mag_filter: i32 = 0;
            let mut min_filter: i32 = 0;

            gl_check!(gl::GetTexParameteriv(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                &mut mag_filter
            ));

            gl_check!(gl::GetTexParameteriv(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                &mut min_filter
            ));

            assert_eq!(mag_filter, min_filter, "buggy graphics driver");

            if mag_filter as gl::types::GLenum == gl::LINEAR {
                true
            }
            else if mag_filter as gl::types::GLenum == gl::NEAREST {
                false
            }
            else {
                panic!("buggy graphics driver");
            }
        };

        smooth
    }

    /// Brief description
    ///
    /// The **set_smooth() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn set_smooth(&mut self, smooth: bool) {
        // Make the OpenGL texture object current (so TexParameteri() operates on it).
        self.bind();

        unsafe {
            let value = if smooth { gl::LINEAR } else { gl::NEAREST };

            gl_check!(gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                value as gl::types::GLint
            ));

            gl_check!(gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                value as gl::types::GLint
            ));
        }
    }

    /// Brief description
    ///
    /// The **is_repeated() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn is_repeated(&self) -> bool {
        // Make the OpenGL texture object current (so GetTexParameteri() operates on it).
        self.bind();

        // Ask OpenGL the smooth property of the OpenGL texture object.
        let repeated = unsafe {
            let mut wrap_s: i32 = 0;
            let mut wrap_t: i32 = 0;

            gl_check!(gl::GetTexParameteriv(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                &mut wrap_s
            ));

            gl_check!(gl::GetTexParameteriv(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                &mut wrap_t
            ));

            assert_eq!(wrap_s, wrap_t, "buggy graphics driver");

            if wrap_s as gl::types::GLenum == gl::REPEAT {
                true
            }
            else if wrap_s as gl::types::GLenum == gl::CLAMP_TO_EDGE {
                false
            }
            else {
                panic!("buggy graphics driver");
            }
        };

        repeated
    }

    /// Brief description
    ///
    /// The **set_repeated() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn set_repeated(&mut self, repeat: bool) {
        self.bind();

        unsafe {
            let value = if repeat { gl::REPEAT } else { gl::CLAMP_TO_EDGE };

            gl_check!(gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                value as gl::types::GLint
            ));

            gl_check!(gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                value as gl::types::GLint
            ));
        }
    }

    pub(crate) fn bind(&self) {
        unsafe {
            gl_check!(gl::BindTexture(gl::TEXTURE_2D, self.object));
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl_check!(gl::DeleteTextures(1, &self.object));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_image() -> Image {
        let pixels = vec![
            Color::BLACK,
            Color::WHITE,
            Color::RED,
            Color::GREEN,
            Color::BLUE,
            Color::TRANSPARENT
        ];
        let size = Size::<usize>::new(3, 2);
        Image::with_pixels(size, pixels).unwrap()
    }

    #[test]
    fn texture_new() {
        let texture = Texture::new();
        assert_eq!(texture.size(), Size::new(0, 0));
        assert_eq!(texture.is_smooth(), false);
        assert_eq!(texture.is_repeated(), false);
    }

    #[test]
    fn texture_with_size() {
        let texture = Texture::with_size(Size::new(25, 50), Color::RED);
        assert_eq!(texture.size(), Size::new(25, 50));
        assert_eq!(texture.is_smooth(), false);
        assert_eq!(texture.is_repeated(), false);

        let image = texture.to_image();
        assert_eq!(image.size(), Size::new(25, 50));
        for pixel in image.pixels().iter() {
            assert_eq!(*pixel, Color::RED);
        }
    }

    #[test]
    fn texture_with_image() {
        let image = make_image();

        let texture = Texture::with_image(&image);
        assert_eq!(texture.size(), Size::new(3, 2));
        assert_eq!(texture.is_smooth(), false);
        assert_eq!(texture.is_repeated(), false);
    }

    #[test]
    fn texture_open() {
        // To be implemented.
    }

    #[test]
    fn texture_resize() {
        // To be implemented.
    }

    #[test]
    fn texture_update_image() {
        // To be implemented.
    }

    #[test]
    fn texture_to_image() {
        let image = make_image();

        let texture = Texture::with_image(&image);
        assert_eq!(texture.to_image(), image);
    }

    #[test]
    fn texture_smooth() {
        let mut texture = Texture::new();
        assert_eq!(texture.is_smooth(), false);

        texture.set_smooth(true);
        assert_eq!(texture.is_smooth(), true);

        texture.set_smooth(false);
        assert_eq!(texture.is_smooth(), false);
    }

    #[test]
    fn texture_repeated() {
        let mut texture = Texture::new();
        assert_eq!(texture.is_repeated(), false);

        texture.set_repeated(true);
        assert_eq!(texture.is_repeated(), true);

        texture.set_repeated(false);
        assert_eq!(texture.is_repeated(), false);
    }

    #[test]
    fn texture_draw() {
        // To be implemented.
    }

    #[test]
    fn texture_bind() {
        // To be implemented.
    }
}
