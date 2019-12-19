// Copyright (c) 2020 - BytePlug
//
// This source file is part of Distilled Multimedia Library which is released
// under the MIT license. Please refer to the LICENSE file that can be found
// at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

//! Give access to the real-time state of gamepads.
//!
//! Additional documentation is to be written here.

/// # An enumeration of the gamepad d-pad directions
///
/// Long description.
///
pub enum DirectionalPad {
    Left,
    Bottom,
    Right,
    Top
}

/// # An enumeration of the gamepad joysticks.
///
/// Long description.
///
pub enum Joystick {
    Left,
    Right
}

/// # An enumeration of the gamepad buttons
///
/// Long description.
///
pub enum Button {
    One,
    Two,
    Three,
    Four,
    L1,
    L2,
    L3,
    R1,
    R2,
    R3,
    Back,
    Start
}

/// # Brief description
///
/// Long description.
///
pub struct Gamepad {}