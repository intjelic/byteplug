// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use std::io;
use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;
use png;
use png::{BitDepth, ColorType};
use crate::geometry::{Position, Size};
use crate::image::Color;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Error {
    InvalidPixels(usize), // Contains the number of expected pixels.
    CorruptedData
}

fn position_to_index(position: &Position<isize>, width: usize) -> usize {
    (position.y * width as isize + position.x) as usize
}

/// Brief description
///
/// The **Image struct** is not documented yet. Pull requests are welcome.
///
/// Below are some notes that should go in the documentation.
///
/// - An empty image has size (0, 0) and is also called 'default image'.
/// - The with_size() constructor goes in pair with size() and resize() methods.
/// - The with_pixels() constructor goes in pair with pixels() and update_pixels(). There are also
///   pixel() and pixel_mut() for per-pixel read and write.
/// - Mention the internal pixel order (left to right, top to bottom).
///
/// **Implementation notes**
///
/// - The interface is far from being completed; it should become a more featured image class that
///   allows common image operations (before they're loaded into texture). But I preferred to keep
///   with something simple. Also see `ingrid` crate.
/// - Supports loading and saving PNG only (for now); will evolve into supporting the most common
///   image format indeed.
/// - I haven't figured out how should be for default numeric types. In the meantimes, the
///   implementation is using usize for sizes and isize for positions. But it might change in the
///   future.
/// - For now, `open()`, `load()` and `save()` methods all return `io::Result` but it should
///   perhaps define a error enums and uses the ErrorKind (see doc).
/// - The default trait should be implemented.
///
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Image {
    size: Size<usize>,
    pixels: Vec<Color>
}

impl Image {

    /// Brief description
    ///
    /// The **new() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn new() -> Image {
        Image {
            size: Size::new(0, 0),
            pixels: Vec::new()
        }
    }

    /// Brief description
    ///
    /// The **with_size() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn with_size(size: Size<usize>, color: Color) -> Image {
        Image {
            size: size,
            pixels: vec![color; size.width * size.height]
        }
    }

    /// Brief description
    ///
    /// The **with_pixels() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn with_pixels(size: Size<usize>, pixels: Vec<Color>) -> Result<Image, Error> {
        let mut image = Image::new();
        image.update_pixels(pixels, size)?;

        Ok(image)
    }

    /// Brief description
    ///
    /// The **open() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Image> {
        let mut reader = File::open(path)?;

        let mut image = Image::new();
        image.load(&mut reader)?;

        Ok(image)
    }

    /// Brief description
    ///
    /// The **size() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn size(&self) -> Size<usize> {
        self.size
    }

    /// Brief description
    ///
    /// The **resize() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn resize(&mut self, size: Size<usize>, color: Color) {
        // I don't like this implementation as it's shifting pixels around and leave the entire
        // image in an inconsistent state (we can't use it for cropping image for example) but it
        // will get fixed later when the implementation is updated to use the ingrid crate.
        self.size = size;

        let length = size.width * size.height;
        self.pixels.resize(length, color);
        self.pixels.reserve_exact(length);
    }

    /// Brief description
    ///
    /// The **load() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn load<R: Read>(&mut self, reader: &mut R) -> io::Result<()> {
        // Read the size and pixels of the image.
        let decoder = png::Decoder::new(reader);
        let (info, mut reader) = decoder.read_info().unwrap();

        let length = info.buffer_size();
        let mut pixels = Vec::<u8>::with_capacity(length);
        pixels.resize(length, 0);

        reader.next_frame(&mut pixels).unwrap();

        // Update the image with the size and pixels.
        assert_eq!(info.bit_depth, BitDepth::Eight);
        assert_eq!(info.color_type, ColorType::RGBA);
        self.size.width = info.width as usize;
        self.size.height = info.height as usize;

        let pixels = unsafe {
            // Ensure the original vector is not dropped.
            let mut pixels_clone = std::mem::ManuallyDrop::new(pixels);
            Vec::from_raw_parts(pixels_clone.as_mut_ptr() as *mut Color,
                                pixels_clone.len() / 4,
                                pixels_clone.capacity() / 4)
        };
        self.pixels = pixels;

        Ok(())
    }

    /// Brief description
    ///
    /// The **save() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn save<W: Write>(&mut self, writer: &mut W) -> io::Result<()> { // todo: should not be `&mut self`
        let mut encoder = png::Encoder::new(writer, self.size.width as _, self.size.height as _);
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        let pixels = unsafe {
            Vec::from_raw_parts(
                self.pixels.as_mut_ptr() as *mut u8,
                self.pixels.len() * 4,
                self.pixels.capacity() * 4
            )
        };
        writer.write_image_data(&pixels).unwrap();
        std::mem::ManuallyDrop::new(pixels);

        Ok(())
    }

    /// Brief description
    ///
    /// The **pixel() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn pixel(&self, position: Position<isize>) -> Option<&Color> {
        self.pixels.get(position_to_index(&position, self.size.width))
    }

    /// Brief description
    ///
    /// The **pixel_mut() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn pixel_mut(&mut self, position: Position<isize>) -> Option<&mut Color> {
        self.pixels.get_mut(position_to_index(&position, self.size.width))
    }

    /// Brief description
    ///
    /// The **pixels() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn pixels(&self) -> &Vec<Color> {
        &self.pixels
    }

    /// Brief description
    ///
    /// The **update_pixels() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn update_pixels(&mut self, mut pixels: Vec<Color>, size: Size<usize>) -> Result<(), Error> {
        // Check if the pixels are valid (expected number of pixels are correct).
        let length = size.width * size.height;
        if pixels.len() != length {
            return Err(Error::InvalidPixels(length))
        };

        // Replace the internal vector of pixels (and make sure the capacity of the vector is same
        // as its length).
        pixels.reserve_exact(length);

        self.size = size;
        self.pixels = pixels;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use std::path::PathBuf;
    use std::io::Cursor;
    use super::*;

    fn get_sample_image_path() -> OsString {
        let mut filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        filename.push("resources/samples.png");

        filename.into_os_string()
    }

    fn make_2x3_random_pixels() -> Vec<Color> {
        vec![
            Color::RED,
            Color::GREEN,
            Color::BLUE,
            Color::WHITE,
            Color::BLACK,
            Color::TRANSPARENT
        ]
    }

    #[test]
    fn image_new() {
        let image = Image::new();
        assert_eq!(image.size(), Size::new(0, 0));
        assert_eq!(image.pixels(), &vec![]);
    }

    #[test]
    fn image_with_size() {
        let size = Size::<usize>::new(2, 3);
        let color = Color::default();

        let image = Image::with_size(size, color);
        assert_eq!(image.size(), size);
        assert_eq!(image.pixels(), &vec![Color::default(); size.width * size.height]);
    }

    #[test]
    fn image_with_pixels() {
        let size = Size::<usize>::new(2, 3);
        let pixels = make_2x3_random_pixels();

        let image = Image::with_pixels(size, pixels.clone()).unwrap();
        assert_eq!(image.size(), size);
        assert_eq!(image.pixels(), &pixels);

        // Testing the creation of an image with an invalid number of pixels
        let invalid_pixels = vec![Color::default(); 5];
        let result = Image::with_pixels(size, invalid_pixels);
        assert_eq!(result.unwrap_err(), Error::InvalidPixels(2 * 3));
    }

    #[test]
    fn image_open() {
        // Creating an image with open() is the same as creating an empty image and loading an image
        // file with load().
        let path = get_sample_image_path();

        let image = Image::open(&path).unwrap();

        let mut other_image = Image::new();
        other_image.load(&mut File::open(&path).unwrap()).unwrap();

        assert_eq!(image.size(), other_image.size());
        assert_eq!(image.pixels(), other_image.pixels());
    }

    #[test]
    fn image_resize() {
        // This unit test will need update when the implementation of the resize() method changes
        // to not leave the pixels in an inconsistent state (see implementation notes).
        let mut image = Image::new();

        assert_eq!(image.size(), Size::new(0, 0));
        image.resize(Size::new(2, 3), Color::default());

        assert_eq!(image.size(), Size::new(2, 3));
        assert_eq!(image.pixels().len(), 6);

        for pixel in image.pixels().iter() {
            assert_eq!(pixel, &Color::default());
        }
    }

    #[test]
    fn image_load() {
        // Testing the load() method consists of checking if the resulting size and pixels of loaded
        // image file are correct. We can't check all the pixels, so checking the length and random
        // pixels instead.
        let path = get_sample_image_path();
        let mut file = File::open(&path).unwrap();

        let mut image = Image::new();
        image.load(&mut file).unwrap();

        assert_eq!(image.size(), Size::new(272, 170));

        assert_eq!(image.pixels().len(), 272 * 170);
        assert_eq!(image.pixel(Position::new(0,   0)).unwrap(),   &Color::rgba(97, 154, 199, 255));
        assert_eq!(image.pixel(Position::new(271, 0)).unwrap(),   &Color::rgba(86, 140, 179, 255));
        assert_eq!(image.pixel(Position::new(0,   169)).unwrap(), &Color::rgba(30,  48,  21, 255));
        assert_eq!(image.pixel(Position::new(271, 169)).unwrap(), &Color::rgba(12,  20,   9, 255));

        // Testing random pixels
        assert_eq!(image.pixel(Position::new(125, 73)).unwrap(),  &Color::rgba(140, 152, 101, 255));
        assert_eq!(image.pixel(Position::new(120, 45)).unwrap(),  &Color::rgba(243, 224, 123, 255));
        assert_eq!(image.pixel(Position::new(109, 21)).unwrap(),  &Color::rgba(135, 181, 217, 255));
        assert_eq!(image.pixel(Position::new( 39, 107)).unwrap(), &Color::rgba(17, 37, 8, 255));
        assert_eq!(image.pixel(Position::new(110, 26)).unwrap(),  &Color::rgba(144, 186, 209, 255));
        assert_eq!(image.pixel(Position::new( 67, 87)).unwrap(),  &Color::rgba(37, 74, 58, 255));
        assert_eq!(image.pixel(Position::new(250, 95)).unwrap(),  &Color::rgba(103, 139, 45, 255));
        assert_eq!(image.pixel(Position::new(205, 94)).unwrap(),  &Color::rgba(19, 52, 41, 255));
        assert_eq!(image.pixel(Position::new( 39, 128)).unwrap(), &Color::rgba(27, 50, 13, 255));
        assert_eq!(image.pixel(Position::new(138, 129)).unwrap(), &Color::rgba(54, 73, 73, 255));
    }

    #[test]
    fn image_save() {
        // If we're able to load the image that we justed saved, and it contains the same pixels,
        // then it's tested. Note that there is likely a better way to implement this test without
        // using a fixed length buffer.
        let path = get_sample_image_path();
        let mut image = Image::open(&path).unwrap(); // todo: it should not be mutable but for now save() needs it

        let buffer: Vec::<u8> = vec![0; 200000];
        let mut cursor = Cursor::new(buffer);

        image.save(&mut cursor).unwrap();

        cursor.set_position(0);

        let mut saved_image = Image::new();
        saved_image.load(&mut cursor).unwrap();

        assert_eq!(image.size(), saved_image.size());
        assert_eq!(image.pixels(), saved_image.pixels());
    }

    #[test]
    fn image_pixels() {
        // This unit test is not only testing the update_pixels() method, but also the getter and
        // setter which are pixel() and pixel_mut().
        let size = Size::new(2, 3);
        let mut pixels = make_2x3_random_pixels();

        let mut image = Image::new();
        image.update_pixels(pixels.clone(), size).unwrap();

        assert_eq!(image.size(), size);
        assert_eq!(image.pixels(), &pixels);

        assert_eq!(image.pixel(Position::new(0, 0)), Some(&Color::RED));
        assert_eq!(image.pixel(Position::new(1, 0)), Some(&Color::GREEN));
        assert_eq!(image.pixel(Position::new(0, 1)), Some(&Color::BLUE));
        assert_eq!(image.pixel(Position::new(1, 1)), Some(&Color::WHITE));
        assert_eq!(image.pixel(Position::new(0, 2)), Some(&Color::BLACK));
        assert_eq!(image.pixel(Position::new(1, 2)), Some(&Color::TRANSPARENT));

        *image.pixel_mut(Position::new(1, 0)).unwrap() = Color::MAGENTA;
        *image.pixel_mut(Position::new(0, 2)).unwrap() = Color::CYAN;
        *image.pixel_mut(Position::new(1, 2)).unwrap() = Color::YELLOW;

        pixels[1] = Color::MAGENTA;
        pixels[4] = Color::CYAN;
        pixels[5] = Color::YELLOW;

        assert_eq!(image.pixels(), &pixels);

        // Testing the update of on an image with an invalid number of pixels (testing both, too
        // much and too little pixels).
        {
            let invalid_pixels = vec![Color::default(); 5];
            let result = image.update_pixels(invalid_pixels, size);
            assert_eq!(result.unwrap_err(), Error::InvalidPixels(2 * 3));
        }

        {
            let invalid_pixels = vec![Color::default(); 7];
            let result = image.update_pixels(invalid_pixels, size);
            assert_eq!(result.unwrap_err(), Error::InvalidPixels(2 * 3));
        }

        // Making sure the image is left untouched.
        assert_eq!(image.size(), size);
        assert_eq!(image.pixels(), &pixels);
    }
}
