// Copyright (c) 2020 - BytePlug
//
// This source file is part of Distilled Multimedia Library which is released
// under the MIT license. Please refer to the LICENSE file that can be found
// at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

//! # Give access to the real-time state of the keyboard.
//!
//! Additional documentation is to be written here.

use std::collections::HashMap;

/// # Brief description.
///
/// The **Component struct** is not documented yet. Pull requests are welcome.
///
pub enum Direction {
    Left, Right, Up, Down
}

/// # An enumeration of the keyboard keys.
///
/// Long description.
///
pub enum Key {
    /// A letter key (A, B, C, ... up to Z)
    Letter(char),
    /// A number key (0, 1, 2, ... up to 9)
    Number(u8),

    /// The escape key
    Escape,
    /// A function key (F1, F2, F3, ... up to F24)
    Function(u8),

    /// The control key (left or right)
    Control {
        /// Left or right version of the key indicator
        left: bool
    },
    /// The shift key (left or right)
    Shift {
        /// Left or right version of the key indicator
        left: bool
    },
    /// The alternate key (left or right)
    Alternate {
        /// Left or right version of the key indicator
        left: bool
    },
    /// The OS-specific key (named either 'Windows' or 'Command')
    System,

    /// One of the arrow key
    Arrow {
        /// The direction of the arrow (left, right, up or down)
        direction: Direction
    },

    /// The ; key
    Semilicon,
    /// The , key
    Comma,
    /// The . key
    Period,
    /// The ' key
    Quote,
    /// The / key
    Slash,
    /// The \ key
    Backslash,
    /// The ~ key
    Tilde,
    /// The = key
    Equal,
    /// The - key
    Hyphen,
    /// The space key
    Space,
    /// The enter/return key
    Enter,
    /// The backspace key
    Backspace,
    /// The tabulation key
    Tab,
    /// The page up key
    PageUp,
    /// The page down key
    PageDown,
    /// The left bracket key
    LeftBracket,
    /// The right bracket key
    RightBracket,

    /// The menu key
    Menu,
    /// The end key
    End,
    /// The home key
    Home,
    /// The insert key
    Insert,
    /// The delete key
    Delete,
    /// The pause key
    Pause,

    /// A numpad number key (0, 1, 2, ... up to 9)
    Numpad(u8),
    /// The + key
    Add,
    /// The - key
    Subtract,
    /// The * key
    Multiply,
    /// The / key
    Divide,

    /// An unknown key (with its OS-specific identifier)
    Unknown(u32)
}

/// # Brief description.
///
/// Long description.
///
pub struct Keyboard {
    pressed_keys: HashMap<Key, ()>
}

impl Keyboard {
    /// Brief description.
    ///
    /// Long description.
    ///
    pub fn is_key_pressed(&self) -> bool {
        false
    }

    /// Brief description.
    ///
    /// Long description.
    ///
    pub fn is_connected() -> bool {
        false
    }
}
