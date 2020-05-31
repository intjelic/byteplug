// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

use std::marker::PhantomData;
use winit;
use winit::platform::desktop::EventLoopExtDesktop;
use glutin;
use crate::geometry::{Position, Size, Vector};
use crate::image::Color;
use crate::draw::{gl, Surface};
use crate::controller::keyboard;
use crate::controller::mouse;
use crate::widget::*;
use crate::application::get_or_create_event_loop;

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

    window: winit::window::Window,
    surface: Surface,

    pub on_move: Option<WidgetMoveFunction<Window<States>, States>>,
    pub on_resize: Option<WidgetResizeFunction<Window<States>, States>>,
    pub on_draw: Option<WidgetDrawFunction<Window<States>, States>>,

    pub on_focus_gain: Option<FocusGainFunction<Window<States>, States>>,
    pub on_focus_lose: Option<FocusLoseFunction<Window<States>, States>>,

    pub on_key_down: Option<KeyDownFunction<Window<States>, States>>,
    pub on_key_up: Option<KeyUpFunction<Window<States>, States>>,
    pub on_character_enter: Option<CharacterEnterFunction<Window<States>, States>>,

    pub on_cursor_enter: Option<CursorEnterFunction<Window<States>, States>>,
    pub on_cursor_leave: Option<CursorLeaveFunction<Window<States>, States>>,
    pub on_cursor_move: Option<CursorMoveFunction<Window<States>, States>>,

    pub on_mouse_down: Option<MouseDownFunction<Window<States>, States>>,
    pub on_mouse_up: Option<MouseUpFunction<Window<States>, States>>,
    pub on_mouse_scroll: Option<MouseScrollFunction<Window<States>, States>>,

    states: PhantomData<States>,
}

impl<States> Window<States> {
    /// Brief description
    ///
    /// The **new() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn new(size: Size) -> Window<States> {
        let event_loop = get_or_create_event_loop();
        let window_builder = winit::window::WindowBuilder::new()
            .with_inner_size(winit::dpi::LogicalSize::new(size.width, size.height));

        let windowed_context = glutin::ContextBuilder::new()
            .build_windowed(window_builder, &event_loop)
            .unwrap();

        let (mut raw_context, window) = unsafe {
            windowed_context.split()
        };

        let surface = Surface::from_window(raw_context, size);

        Window {
            title: String::from(""),
            position: Position::new(0, 0),
            size: size,

            window: window,
            surface: surface,

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

    /// Brief description
    ///
    /// The **surface() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn surface(&mut self) -> &mut Surface {
        &mut self.surface
    }

    /// Handle a "window moved" event
    ///
    /// This function does something which is not documented yet.
    ///
    fn handle_move_event(&mut self, states: &mut States, physical_position: &winit::dpi::PhysicalPosition<i32>) {
        let position = Position::new(physical_position.x as i32, physical_position.y as i32);

        match self.on_move {
            None => (),
            Some(function) => { function(self, position, states); }
        }

        self.position = position;
    }

    /// Handle a "window resized" event
    ///
    /// This function does something which is not documented yet.
    ///
    fn handle_resize_event(&mut self, states: &mut States, physical_size: &winit::dpi::PhysicalSize<u32>) {
        let size = Size::new(physical_size.width as i32, physical_size.height as i32);

        match self.on_resize {
            None => (),
            Some(function) => { function(self, size, states); }
        }

        self.size = size;
    }

    /// Handle a "window redraw" event
    ///
    /// This function does something which is not documented yet.
    ///
    fn handle_redraw_event(&mut self, states: &mut States) -> winit::event_loop::ControlFlow {
        let mut pixels = Vec::<Color>::new();

        match self.on_draw {
            None => (),
            Some(function) => { function(self, &mut pixels, states); }
        };

        winit::event_loop::ControlFlow::Wait
    }

    /// Handle a "window focus" event related to this window
    ///
    /// This function does something which is not documented yet.
    ///
    fn handle_focus_event(&mut self, states: &mut States, has_focus: &bool) {
        if *has_focus {
            match self.on_focus_gain {
                None => (),
                Some(function) => { function(self, states); }
            }
        }
        else {
            match self.on_focus_lose {
                None => (),
                Some(function) => { function(self, states); }
            }
        }
    }

    /// Handle a keyboard-related event related to this window
    ///
    /// This function does something which is not documented yet.
    ///
    fn handle_keyboard_input(&mut self, states: &mut States, input: &winit::event::KeyboardInput, is_synthetic: &bool) {
        let key = map_key_code(input.virtual_keycode);

        let modifiers = keyboard::Modifiers {
            control: input.modifiers.ctrl(),
            shift: input.modifiers.shift(),
            alternate: input.modifiers.alt(),
            system: input.modifiers.logo()
        };

        let function = match input.state {
                winit::event::ElementState::Pressed => self.on_key_down,
                winit::event::ElementState::Released => self.on_key_up
        };

        match function {
            None => (),
            Some(ref function) => { function(self, key, modifiers, states); }
        }
    }

    /// Handle a "character received" event related to this window
    ///
    /// This function does something which is not documented yet.
    ///
    fn handle_received_character(&mut self, states: &mut States, character: &char) {
        match self.on_character_enter {
            None => (),
            Some(function) => { function(self, *character, states); }
        }
    }

    /// Handle a "cursor entered" event related to this window
    ///
    /// This function does something which is not documented yet.
    ///
    fn handle_cursor_entered(&mut self, states: &mut States) {
        let position = Position::new(0, 0);

        match self.on_cursor_enter {
            None => (),
            Some(function) => { function(self, position, states); }
        }
    }

    /// Handle s "cursor left" event related to this window
    ///
    /// This function does something which is not documented yet.
    ///
    fn handle_cursor_left(&mut self, states: &mut States) {
        let position = Position::new(0, 0);

        match self.on_cursor_leave {
            None => (),
            Some(function) => { function(self, position, states); }
        }
    }

    /// Handle a "cursor moved" event related to this window
    ///
    /// This function does something which is not documented yet.
    ///
    fn handle_cursor_moved(&mut self,
                           states: &mut States,
                           position: &winit::dpi::PhysicalPosition<f64>,
                           _modifiers: &winit::event::ModifiersState) {
        let position = Position::new(position.x as i32, position.y as i32);
        let vector = Vector::new(0., 0.);

        match self.on_cursor_move {
            None => (),
            Some(function) => { function(self, position, vector, states); }
        }
    }

    /// Handle a keyboard-related event related to this window
    ///
    /// This function does something which is not documented yet.
    ///
    fn handle_mouse_input(&mut self,
                          states: &mut States,
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
                            states: &mut States,
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
            self.on_mouse_scroll.unwrap()(self, mouse::Wheel::Vertical, movement.y, states);
        }

        if movement.x != 0. {
            self.on_mouse_scroll.unwrap()(self, mouse::Wheel::Horizontal, movement.x, states);
        }
    }

    /// Handle all events related to this window
    ///
    /// This function handles all events read by the event loop which are directly related to this
    /// window. It reads the event instance (the `winit` event is an enumeration that contains the
    /// data of the event) and just unpacks it to send the event data to a more specific handler
    /// function, possibly discarding values that are not needed.
    ///
    fn handle_window_event(&mut self, states: &mut States, event: &winit::event::WindowEvent)
                            -> winit::event_loop::ControlFlow {
        match event {
            winit::event::WindowEvent::Moved(physical_position) => {
                self.handle_move_event(states, physical_position);
            },
            winit::event::WindowEvent::Resized(physical_size) => {
                self.handle_resize_event(states, physical_size);
            },
            winit::event::WindowEvent::CloseRequested => {
                return winit::event_loop::ControlFlow::Exit;
            },
            winit::event::WindowEvent::Focused(has_focus) => {
                self.handle_focus_event(states, has_focus);
            },
            winit::event::WindowEvent::KeyboardInput { device_id: _, input, is_synthetic } => {
                self.handle_keyboard_input(states, input, is_synthetic);
            },
            winit::event::WindowEvent::ReceivedCharacter(character) => {
                self.handle_received_character(states, character);
            },
            winit::event::WindowEvent::CursorEntered { device_id: _ } => {
                self.handle_cursor_entered(states);
            },
            winit::event::WindowEvent::CursorLeft { device_id: _ } => {
                self.handle_cursor_left(states);
            },
            winit::event::WindowEvent::CursorMoved { device_id: _, position, modifiers } => {
                self.handle_cursor_moved(states, position, modifiers);
            },
            winit::event::WindowEvent::MouseInput { device_id: _, state, button, modifiers }=> {
                self.handle_mouse_input(states, state, button, modifiers);
            },
            winit::event::WindowEvent::MouseWheel { device_id: _, delta, phase, modifiers } => {
                self.handle_mouse_wheel(states, delta, phase, modifiers);
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
    pub fn run(&mut self, states: &mut States) {
        let event_loop = get_or_create_event_loop();
        event_loop.run_return(move |event, _, control_flow| {
            *control_flow = match event {
                winit::event::Event::LoopDestroyed => return,
                winit::event::Event::WindowEvent { ref event, .. } => self.handle_window_event(states, event),
                winit::event::Event::RedrawRequested(_) => self.handle_redraw_event(states),
                _ => winit::event_loop::ControlFlow::Wait,
            }
        });
    }
}
