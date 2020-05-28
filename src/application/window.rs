// Copyright (c) 2020 - BytePlug
//
// This source file is part of Distilled Multimedia Library which is released
// under the MIT license. Please refer to the LICENSE file that can be found
// at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use std::marker::PhantomData;
use winit;
use winit::platform::desktop::EventLoopExtDesktop;
use crate::geometry::{Position, Size, Vector};
use crate::image::Color;
use crate::controller::keyboard;
use crate::controller::mouse;
use crate::widget::*;

fn map_key_code(key: Option<winit::event::VirtualKeyCode>) -> keyboard::Key {
    // todo: finish implementation
    match key {
        Some(value) => {
            match value {
                winit::event::VirtualKeyCode::Key1 => keyboard::Key::Number(1),
                winit::event::VirtualKeyCode::Numpad9 => keyboard::Key::Numpad(9),

                _ => keyboard::Key::Unknown(0)
            }
        },
        None => keyboard::Key::Unknown(0)
    }
}

/// Brief description
///
/// The **Window struct** is not documented yet. Pull requests are welcome.
///
/// **Implementation notes**
///
/// - Cursor enter/leave doesn't compute the cursor position
/// - Test and document (possibly improve interface and uniformize values) of mouse wheel scroll
///   values
/// - The map_key_code() is not complete (todo).
///
pub struct Window<States> {
    title: String,
    position: Position,
    size: Size,

    pub on_move: Option<WidgetMoveFunction>,
    pub on_resize: Option<WidgetResizeFunction>,
    pub on_draw: Option<WidgetDrawFunction>,

    pub on_focus_gain: Option<FocusGainFunction>,
    pub on_focus_lose: Option<FocusLoseFunction>,

    pub on_key_down: Option<KeyDownFunction>,
    pub on_key_up: Option<KeyUpFunction>,
    pub on_character_enter: Option<CharacterEnterFunction>,

    pub on_cursor_enter: Option<CursorEnterFunction>,
    pub on_cursor_leave: Option<CursorLeaveFunction>,
    pub on_cursor_move: Option<CursorMoveFunction>,

    pub on_mouse_down: Option<MouseDownFunction>,
    pub on_mouse_up: Option<MouseUpFunction>,
    pub on_mouse_scroll: Option<MouseScrollFunction>,

    states: PhantomData<States>,
}

impl<States> Window<States> {
    pub fn new() -> Window<States> {
        Window {
            title: String::from(""),
            position: Position::new(0, 0),
            size: Size::new(0, 0),

            on_move: None,
            on_resize: None,
            on_draw: None,

            on_focus_gain: None,
            on_focus_lose: None,

            on_key_down: None,
            on_key_up: None,
            on_character_enter: None,

            on_cursor_enter: None,
            on_cursor_leave: None,
            on_cursor_move: None,

            on_mouse_down: None,
            on_mouse_up: None,
            on_mouse_scroll: None,

            states: PhantomData
        }
    }

    /// Handle a "window moved" event
    ///
    /// This function does something which is not documented yet.
    ///
    fn handle_move_event(&mut self, logical_position: &winit::dpi::LogicalPosition) {
        let position = Position::new(logical_position.x as i32, logical_position.y as i32);

        match self.on_move {
            None => (),
            Some(function) => { function(position); }
        }

        self.position = position;
    }

    /// Handle a "window resized" event
    ///
    /// This function does something which is not documented yet.
    ///
    fn handle_resize_event(&mut self, logical_size: &winit::dpi::LogicalSize) {
        let size = Size::new(logical_size.width as i32, logical_size.height as i32);

        match self.on_resize {
            None => (),
            Some(function) => { function(size); }
        }

        self.size = size;
    }

    /// Handle a "window redraw" event
    ///
    /// This function does something which is not documented yet.
    ///
    fn handle_redraw_event(&mut self) {
        let mut pixels = Vec::<Color>::new();

        match self.on_draw {
            None => (),
            Some(function) => { function(&mut pixels); }
        }
    }

    /// Handle a "window focus" event related to this window
    ///
    /// This function does something which is not documented yet.
    ///
    fn handle_focus_event(&mut self, has_focus: &bool) {
        if *has_focus {
            match self.on_focus_gain {
                None => (),
                Some(function) => { function(); }
            }
        }
        else {
            match self.on_focus_lose {
                None => (),
                Some(function) => { function(); }
            }
        }
    }

    /// Handle a keyboard-related event related to this window
    ///
    /// This function does something which is not documented yet.
    ///
    fn handle_keyboard_input(&mut self, input: &winit::event::KeyboardInput, is_synthetic: &bool) {
        let key = map_key_code(input.virtual_keycode);

        let modifiers = keyboard::Modifiers {
            control: input.modifiers.ctrl,
            shift: input.modifiers.shift,
            alternate: input.modifiers.alt,
            system: input.modifiers.logo
        };

        let function = match input.state {
                winit::event::ElementState::Pressed => self.on_key_down,
                winit::event::ElementState::Released => self.on_key_up
        };

        match function {
            None => (),
            Some(ref function) => { function(key, modifiers); }
        }
    }

    /// Handle a "character received" event related to this window
    ///
    /// This function does something which is not documented yet.
    ///
    fn handle_received_character(&mut self, character: &char) {
        match self.on_character_enter {
            None => (),
            Some(function) => { function(*character); }
        }
    }

    /// Handle a "cursor entered" event related to this window
    ///
    /// This function does something which is not documented yet.
    ///
    fn handle_cursor_entered(&mut self) {
        let position = Position::new(0, 0);

        match self.on_cursor_enter {
            None => (),
            Some(function) => { function(position); }
        }
    }

    /// Handle s "cursor left" event related to this window
    ///
    /// This function does something which is not documented yet.
    ///
    fn handle_cursor_left(&mut self) {
        let position = Position::new(0, 0);

        match self.on_cursor_leave {
            None => (),
            Some(function) => { function(position); }
        }
    }

    /// Handle a "cursor moved" event related to this window
    ///
    /// This function does something which is not documented yet.
    ///
    fn handle_cursor_moved(&mut self,
                           position: &winit::dpi::LogicalPosition,
                           _modifiers: &winit::event::ModifiersState) {
        let position = Position::new(position.x as i32, position.y as i32);
        let vector = Vector::new(0., 0.);

        match self.on_cursor_move {
            None => (),
            Some(function) => { function(position, vector); }
        }
    }

    /// Handle a keyboard-related event related to this window
    ///
    /// This function does something which is not documented yet.
    ///
    fn handle_mouse_input(&mut self,
                          state: &winit::event::ElementState,
                          button: &winit::event::MouseButton,
                          modifiers: &winit::event::ModifiersState) {
        // To be implemented...
    }

    /// Handle a mouse wheel event related to this window
    ///
    /// This function calls the mouse scroll callback function with their values. The delta value is
    /// a 2D position describing whether it was a horizontal or vertical scroll. If values are
    /// provided on both axis, the user function is called twice with different values.
    ///
    /// **Notes**
    ///
    /// - For now, the value is not precise, testing should be done in order to determine what
    ///
    fn handle_mouse_wheel(&mut self,
                            delta: &winit::event::MouseScrollDelta,
                            _phase: &winit::event::TouchPhase,
                            _modifiers: &winit::event::ModifiersState) {
        // todo: normalize the 2D scroll movement to work with a single value later
        let movement = match delta {
            winit::event::MouseScrollDelta::LineDelta(x, y) => {
                Position::<f64>::new(*x as f64, *y as f64)
            },
            winit::event::MouseScrollDelta::PixelDelta(logical_position) => {
                Position::<f64>::new(logical_position.x, logical_position.y)
            }
        };

        if movement.y != 0. {
            self.on_mouse_scroll.unwrap()(mouse::Wheel::Vertical, movement.y);
        }

        if movement.x != 0. {
            self.on_mouse_scroll.unwrap()(mouse::Wheel::Horizontal, movement.x);
        }
    }

    /// Handle all events related to this window
    ///
    /// This function handles all events read by the event loop which are directly related to this
    /// window. It reads the event instance (the `winit` event is an enumeration that contains the
    /// data of the event) and just unpacks it to send the event data to a more specific handler
    /// function, possibly discarding values that are not needed.
    ///
    fn handle_window_event(&mut self, event: &winit::event::WindowEvent)
                            -> winit::event_loop::ControlFlow {
        match event {
            winit::event::WindowEvent::Moved(logical_position) => {
                self.handle_move_event(logical_position);
            },
            winit::event::WindowEvent::Resized(logical_size) => {
                self.handle_resize_event(logical_size);
            },
            winit::event::WindowEvent::RedrawRequested => {
                self.handle_redraw_event();
            },
            winit::event::WindowEvent::CloseRequested => {
                return winit::event_loop::ControlFlow::Exit;
            },
            winit::event::WindowEvent::Focused(has_focus) => {
                self.handle_focus_event(has_focus);
            },
            winit::event::WindowEvent::KeyboardInput { device_id: _, input, is_synthetic } => {
                self.handle_keyboard_input(input, is_synthetic);
            },
            winit::event::WindowEvent::ReceivedCharacter(character) => {
                self.handle_received_character(character);
            },
            winit::event::WindowEvent::CursorEntered { device_id: _ } => {
                self.handle_cursor_entered();
            },
            winit::event::WindowEvent::CursorLeft { device_id: _ } => {
                self.handle_cursor_left();
            },
            winit::event::WindowEvent::CursorMoved { device_id: _, position, modifiers } => {
                self.handle_cursor_moved(position, modifiers);
            },
            winit::event::WindowEvent::MouseInput { device_id: _, state, button, modifiers }=> {
                self.handle_mouse_input(state, button, modifiers);
            },
            winit::event::WindowEvent::MouseWheel { device_id: _, delta, phase, modifiers } => {
                self.handle_mouse_wheel(delta, phase, modifiers);
            },
            _ => ()
        };

        winit::event_loop::ControlFlow::Wait
    }

    /// Handle window-related event triggered by the event loop
    ///
    /// This function is not documented yet.
    ///
    /// **Notes**
    ///
    /// - The implementation doesn't handle multiple windows; in the case of multiple windows, the
    ///   user is expected to run them in different loop bu there will be a single `winit` event
    ///   loop.
    ///
    pub fn run(mut self, _states: &mut States) {
        let mut event_loop = winit::event_loop::EventLoop::new();
        let _window = winit::window::WindowBuilder::new().build(&event_loop).unwrap();

        event_loop.run_return(move |event, _, control_flow| {
            *control_flow = match event {
                winit::event::Event::LoopDestroyed => return,
                winit::event::Event::WindowEvent { ref event, .. } => self.handle_window_event(event),
                _ => winit::event_loop::ControlFlow::Wait,
            }
        });
    }
}
