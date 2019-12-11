// Copyright (c) 2020 - BytePlug
//
// This source file is part of Distilled Multimedia Library which is released
// under the MIT license. Please refer to the LICENSE file that can be found
// at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

//! # Distilled Multimedia Library
//!
//! **Distill** is a multimedia library for the Rust language which aims to
//! provide a decent framework to write applications (or games) for desktop (or
//! mobile).
//!
//! While it appears to feature tools in many areas (animation, ui, etc.), it
//! actually provides the bare minimum and each are carefully crafted to be
//! extended and integrate well with external libraries.
//! For instance, if you had a bad experience with the thousands of poorly
//! designed GUI toolkits out there, you might be plaisantly surprised by the
//! **ui** momdule.
//!
//! It was heavily inspired by similar framework (like SFML) and other
//! extensions developed by the community (Thor) coupled with my decade of
//! experience in the field and my relentless perfectionism.
pub mod geometry;
pub mod renderer;
pub mod animation;

pub mod audio;
pub mod video;

pub mod input;
pub mod application;

pub mod ui;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
