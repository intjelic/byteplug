// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

//! User devices input handling utilities
//!
//! Controller is a rather broad word but in this context, it refers to **user
//! input** only. It includes all the physical devices which are used to drive
//! the application, such as pluggable keyboards, mouses and gamepads, touch
//! screens and the built-in sensors.
pub mod keyboard;
pub mod mouse;
pub mod gamepad;
pub mod touchpad;
pub mod touchscreen;
pub mod sensor;
pub mod tablet;
