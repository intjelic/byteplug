// Copyright (c) 2020 - BytePlug
//
// This source file is part of Distilled Multimedia Library which is released
// under the MIT license. Please refer to the LICENSE file that can be found
// at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use crate::geometry::{Position, Size, Vector};
use crate::controller::keyboard::{Key, Modifiers};
use crate::controller::mouse::{Button, Wheel};
use crate::image::Color;

/// The widget move callback function
///
/// This function is called whenever the widget position changes. It's called
/// with the new widget position relative to its parent.
///
/// * `position` - The new widget position (relative to its parent).
///
pub type WidgetMoveFunction = fn(position: Position) -> ();

/// The widget resize callback function
///
/// This function is called whenever the widget size changes. It's called with
/// the new widget size.
///
/// * `size` - The new widget size.
///
pub type WidgetResizeFunction = fn(size: Size) -> ();

/// The widget redraw callback function
///
/// This function is called whenever the widget must be redrawn. It's called
/// with a mutable pointer to the widget pixels which are expected to be
/// updated. The widget pixels is expressed as an array of colors with a length
/// equal to "width x height" of the widget size.
///
/// * `pixels` - The widget pixels to be updated.
///
/// Note that the signature of this function will evolve to be more convenient
/// to work with, and to get rid of dependency on the `Color` struct (the
/// widget module should not depend on the image module).
///
pub type WidgetDrawFunction = fn(pixels: &mut Vec<Color>) -> ();

/// The focus gain callback function.
///
/// This function is called whenever the widget focus is gained. Only one widget
/// or no widget is focused at a time and a focused widget receives the keyboard
/// input. Usually it's visually decorated to indicate it's the focused widget.
///
pub type FocusGainFunction = fn() -> ();

/// The focus gain callback function.
///
/// This function is called whenever the widget focus is lost. The widget does
/// not receive the keyboard input until it gains the focus back.
///
pub type FocusLoseFunction = fn() -> ();

/// The keyboard key down callback function.
///
/// This function is called whenever a keyboard key is pressed when the widget
/// is focused. It's called with the identifier of the key being pressed and the
/// current keyboard modifiers.
///
/// * `key` - The identifier of the key being pressed.
/// * `modifiers` - The current keyboard modifiers.
///
pub type KeyDownFunction = fn(key: Key, modifiers: Modifiers) -> ();

/// The keyboard key up callback function.
///
/// This function is called whenever a keyboard key is released when the widget
/// is focused. It's called with the identifier of the key being released and
/// the current keyboard modifiers.
///
/// * `key` - The identifier of the key being released.
/// * `modifiers` - The current keyboard modifiers.
///
pub type KeyUpFunction = fn(key: Key, modifiers: Modifiers) -> ();

/// The character enter callback function.
///
/// This function is called whenever a character is generated, usually after a
/// sequence of keys are pressed. It's the underlying operating system that
/// computes its unicode scalar value according to the keyboard layout settings
/// of the user.
///
/// Note that it doesn't keep the key down/up counter-part callback functions
/// from being called. For instance, pressing the 'A' key on the keyboard will
/// likely generate a key down event with the key identifier 'A' **and** the 'A'
/// character... unless it's configured to generate a different character, like
/// for a Russian keyboard layout for example.
///
/// * `character` - The character entered as a unicode scalar value.
///
pub type CharacterEnterFunction = fn(character: char) -> ();

/// The cursor enter callback function.
///
/// This function is called whenever the mouse cursor enters the widget area.
/// It's called with the position of the cursor relative to the widget top-left
/// corner.
///
/// * `position` - The cursor position (relative to the top-left corner).
///
pub type CursorEnterFunction = fn(position: Position) -> ();

/// The cursor leave callback function
///
/// This function is called whenever the mouse cursor leaves the widget area.
/// It's called with the position of the cursor relative to the widget top-left
/// corner.
///
/// * `position` - The cursor position (relative to the top-left corner).
///
pub type CursorLeaveFunction = fn(position: Position) -> ();

/// The cursor move callback function
///
/// This function is called whenever the mouse cursor moves while it is on the
/// widget area, or when the widget is grabbed. It's called with the position
/// of the cursor relative to the widget top-left corner and a vector
/// describing the cursor movement in pixels coordinate.
///
/// * `position` - The new cursor position (relative to the top-left corner).
/// * `movement` - The movement of the cursor in pixels coordinate.
///
pub type CursorMoveFunction = fn(position: Position, movement: Vector) -> ();

/// The mouse button down callback function
///
/// This function is called whenever a mouse button is pressed when the cursor
/// is on the widget area; the widget is then marked as grabbed by this button.
/// It's called with the identifier of the button being pressed and the current
/// cursor position.
///
/// * `button` - The identifier of the button being pressed.
/// * `position` - The current cursor position.
///
pub type MouseDownFunction = fn(button: Button, position: Position) -> ();

/// The mouse button up callback function
///
/// This function is called whenever a mouse button is released when the widget
/// was grabbed by this very same button. It's called with the identifier of the
/// button being released and the current cursor position relative to the
/// top-left corner of the widget.
///
/// * `button` - The identifier of the button being released.
/// * `position` - The current cursor position (relative to the top-left corner).
///
pub type MouseUpFunction = fn(button: Button, position: Position) -> ();

/// The mouse wheel scroll callback function
///
/// This function is called whenever the wheel of the mouse is scrolled when the
/// cursor is on the widget area. It's called with the identifier of the wheel
/// which can be either horizontal or vertical and the movement of the wheel in
/// pixels.
///
/// Note that the movement value is not precised; work towards testing and
/// documenting, and possibly improving the interface and uniformizing the
/// values, will be done.
///
/// * `wheel` - The identifier of the wheel being scrolled.
/// * `movement` - The movement of the wheel expressed in pixels
///
pub type MouseScrollFunction = fn(wheel: Wheel, movement: f64) -> ();

/// Brief description
///
/// The **Widget trait** is not documented yet. Pull requests are welcome.
///
pub trait Widget {
}
