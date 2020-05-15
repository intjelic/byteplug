// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

//! # Byteplug - Minimalistic Multimedia Library
//!
//! **Byteplug** is a multimedia library for the Rust language which aims to
//! provide a decent framework with minimal set of tools to write applications
//! (or games) for desktop (or mobile). This is in the same essence as the SDL
//! and SFML frameworks.
//!
//! While it appears to cover many areas (animation, graphical user interface,
//! etc.), it actually provides the bare minimum functionnalities and each tool
//! is carefully crafted to work together, to be extended and to integrate well
//! with external libraries. For instance, if you had a bad experience with the
//! thousands of poorly designed GUI toolkits out there, you might be plaisantly
//! surprised by the **widget module** which implements the core logic without
//! defining any default appearance, and leaves you with a ready-to-implement
//! environment to create the next full-fledged GUI framework like GTK or Qt.
//!
//! # Showcase sample
//!
//! This sample showcases how to create a single-window cross-platform
//! application.
//!
//! ```
//! // To be written...
//! ```
//!
//! It was heavily inspired by similar frameworks (like SFML) and other
//! extensions developed by the community (Thor) coupled with my decade of
//! experience in the field of video games and my obsession for perfection.
//!
//! I'm open to suggestions and contributions. I would gladly discuss every
//! design decisions that were taken and adjust the framework accordingly.


#[link(name="GLESv2")] extern {}

pub mod image;
pub mod audio;
pub mod video;

pub mod geometry;
pub mod draw;
pub mod animation;

pub mod controller;
pub mod application;
pub mod widget;

pub mod game;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
