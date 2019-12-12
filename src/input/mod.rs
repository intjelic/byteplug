// Copyright (c) 2020 - BytePlug
//
// This source file is part of Distilled Multimedia Library which is released
// under the MIT license. Please refer to the LICENSE file that can be found
// at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

//! The input module
//!
//! Input is a rather broad word but in this context, it refers to **user
//! input** only. It includes all the physical devices which are used to drive
//! the application, such as pluggable keyboards, mouses and gamepads, touch
//! screens and the built-in sensors.
include!("keyboard.rs");
include!("mouse.rs");
include!("gamepad.rs");
include!("touch.rs");
include!("sensor.rs");