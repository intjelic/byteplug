// Copyright (c) 2020 - BytePlug
//
// This source file is part of Distilled Multimedia Library which is released
// under the MIT license. Please refer to the LICENSE file that can be found
// at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

//! Graphical user interface functionalities
//!
//! Additional documentation is to be written here.
mod layout;
mod component;
mod widget;

pub use layout::Layout;
pub use component::Component;
pub use widget::{WidgetMoveFunction, WidgetResizeFunction, WidgetDrawFunction};
pub use widget::{FocusGainFunction, FocusLoseFunction};
pub use widget::{KeyDownFunction, KeyUpFunction, CharacterEnterFunction};
pub use widget::{CursorEnterFunction, CursorLeaveFunction, CursorMoveFunction};
pub use widget::{MouseDownFunction, MouseUpFunction, MouseScrollFunction};
pub use widget::Widget;
