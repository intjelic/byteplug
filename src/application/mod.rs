// Copyright (c) 2020 - BytePlug
//
// This source file is part of Distilled Multimedia Library which is released
// under the MIT license. Please refer to the LICENSE file that can be found
// at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

//! Cross-platform application utilities
//!
//! Additional documentation is to be written here.
mod event_loop;
mod window;
mod application;

pub use event_loop::get_or_create_event_loop;
pub use window::Window;
pub use application::Application;
