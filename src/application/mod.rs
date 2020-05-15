// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

//! Cross-platform application utilities
//!
//! Additional documentation is to be written here.
pub(crate) use event_loop::get_or_create_event_loop;

mod event_loop;
mod window;
mod application;

pub use window::Window;
pub use application::Application;
