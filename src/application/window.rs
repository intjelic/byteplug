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
use glutin::{GlProfile, GlRequest, Api};
use crate::geometry::{Position, Size, Vector};
use crate::image::Color;
use crate::draw::get_or_create_context;
use crate::draw::Surface;
use crate::controller::keyboard;
use crate::controller::mouse;
use crate::widget::*;
use crate::application::get_or_create_event_loop;

fn map_key_code(key: Option<winit::event::VirtualKeyCode>) -> keyboard::Key {
    // Todo; associate the native OS identifier when using Unknown variant.
    match key {
        Some(value) => {
            match value {
                winit::event::VirtualKeyCode::A => keyboard::Key::Letter('A'),
                winit::event::VirtualKeyCode::B => keyboard::Key::Letter('B'),
                winit::event::VirtualKeyCode::C => keyboard::Key::Letter('C'),
                winit::event::VirtualKeyCode::D => keyboard::Key::Letter('D'),
                winit::event::VirtualKeyCode::E => keyboard::Key::Letter('E'),
                winit::event::VirtualKeyCode::F => keyboard::Key::Letter('F'),
                winit::event::VirtualKeyCode::G => keyboard::Key::Letter('G'),
                winit::event::VirtualKeyCode::H => keyboard::Key::Letter('H'),
                winit::event::VirtualKeyCode::I => keyboard::Key::Letter('I'),
                winit::event::VirtualKeyCode::J => keyboard::Key::Letter('J'),
                winit::event::VirtualKeyCode::K => keyboard::Key::Letter('K'),
                winit::event::VirtualKeyCode::L => keyboard::Key::Letter('L'),
                winit::event::VirtualKeyCode::M => keyboard::Key::Letter('M'),
                winit::event::VirtualKeyCode::N => keyboard::Key::Letter('N'),
                winit::event::VirtualKeyCode::O => keyboard::Key::Letter('O'),
                winit::event::VirtualKeyCode::P => keyboard::Key::Letter('P'),
                winit::event::VirtualKeyCode::Q => keyboard::Key::Letter('Q'),
                winit::event::VirtualKeyCode::R => keyboard::Key::Letter('R'),
                winit::event::VirtualKeyCode::S => keyboard::Key::Letter('S'),
                winit::event::VirtualKeyCode::T => keyboard::Key::Letter('T'),
                winit::event::VirtualKeyCode::U => keyboard::Key::Letter('U'),
                winit::event::VirtualKeyCode::V => keyboard::Key::Letter('V'),
                winit::event::VirtualKeyCode::W => keyboard::Key::Letter('W'),
                winit::event::VirtualKeyCode::X => keyboard::Key::Letter('X'),
                winit::event::VirtualKeyCode::Y => keyboard::Key::Letter('Y'),
                winit::event::VirtualKeyCode::Z => keyboard::Key::Letter('Z'),
                winit::event::VirtualKeyCode::Key1 => keyboard::Key::Number(1),
                winit::event::VirtualKeyCode::Key2 => keyboard::Key::Number(2),
                winit::event::VirtualKeyCode::Key3 => keyboard::Key::Number(3),
                winit::event::VirtualKeyCode::Key4 => keyboard::Key::Number(4),
                winit::event::VirtualKeyCode::Key5 => keyboard::Key::Number(5),
                winit::event::VirtualKeyCode::Key6 => keyboard::Key::Number(6),
                winit::event::VirtualKeyCode::Key7 => keyboard::Key::Number(7),
                winit::event::VirtualKeyCode::Key8 => keyboard::Key::Number(8),
                winit::event::VirtualKeyCode::Key9 => keyboard::Key::Number(9),
                winit::event::VirtualKeyCode::Key0 => keyboard::Key::Number(0),

                winit::event::VirtualKeyCode::Escape => keyboard::Key::Escape,
                winit::event::VirtualKeyCode::F1 => keyboard::Key::Function(1),
                winit::event::VirtualKeyCode::F2 => keyboard::Key::Function(2),
                winit::event::VirtualKeyCode::F3 => keyboard::Key::Function(3),
                winit::event::VirtualKeyCode::F4 => keyboard::Key::Function(4),
                winit::event::VirtualKeyCode::F5 => keyboard::Key::Function(5),
                winit::event::VirtualKeyCode::F6 => keyboard::Key::Function(6),
                winit::event::VirtualKeyCode::F7 => keyboard::Key::Function(7),
                winit::event::VirtualKeyCode::F8 => keyboard::Key::Function(8),
                winit::event::VirtualKeyCode::F9 => keyboard::Key::Function(9),
                winit::event::VirtualKeyCode::F10 => keyboard::Key::Function(10),
                winit::event::VirtualKeyCode::F11 => keyboard::Key::Function(11),
                winit::event::VirtualKeyCode::F12 => keyboard::Key::Function(12),
                winit::event::VirtualKeyCode::F13 => keyboard::Key::Function(13),
                winit::event::VirtualKeyCode::F14 => keyboard::Key::Function(14),
                winit::event::VirtualKeyCode::F15 => keyboard::Key::Function(15),
                winit::event::VirtualKeyCode::F16 => keyboard::Key::Function(16),
                winit::event::VirtualKeyCode::F17 => keyboard::Key::Function(17),
                winit::event::VirtualKeyCode::F18 => keyboard::Key::Function(18),
                winit::event::VirtualKeyCode::F19 => keyboard::Key::Function(19),
                winit::event::VirtualKeyCode::F20 => keyboard::Key::Function(20),
                winit::event::VirtualKeyCode::F21 => keyboard::Key::Function(21),
                winit::event::VirtualKeyCode::F22 => keyboard::Key::Function(22),
                winit::event::VirtualKeyCode::F23 => keyboard::Key::Function(23),
                winit::event::VirtualKeyCode::F24 => keyboard::Key::Function(24),

                winit::event::VirtualKeyCode::Tab => keyboard::Key::Tab,
                winit::event::VirtualKeyCode::Back => keyboard::Key::Backspace,
                winit::event::VirtualKeyCode::Return => keyboard::Key::Enter,
                winit::event::VirtualKeyCode::Space => keyboard::Key::Space,
                winit::event::VirtualKeyCode::LControl => keyboard::Key::Control { left: true },
                winit::event::VirtualKeyCode::RControl => keyboard::Key::Control { left: false },
                winit::event::VirtualKeyCode::LShift => keyboard::Key::Shift { left: true },
                winit::event::VirtualKeyCode::RShift => keyboard::Key::Shift { left: false },
                winit::event::VirtualKeyCode::LAlt => keyboard::Key::Alternate { left: true },
                winit::event::VirtualKeyCode::RAlt => keyboard::Key::Alternate { left: false },
                winit::event::VirtualKeyCode::Comma => keyboard::Key::Comma,
                winit::event::VirtualKeyCode::Semicolon => keyboard::Key::Semicolon,
                winit::event::VirtualKeyCode::Apostrophe => keyboard::Key::Quote,
                winit::event::VirtualKeyCode::Slash => keyboard::Key::Slash,
                winit::event::VirtualKeyCode::Backslash => keyboard::Key::Backslash,
                winit::event::VirtualKeyCode::LBracket => keyboard::Key::LeftBracket,
                winit::event::VirtualKeyCode::RBracket => keyboard::Key::RightBracket,

                winit::event::VirtualKeyCode::Period => keyboard::Key::Period,
                winit::event::VirtualKeyCode::Subtract => keyboard::Key::Subtract,
                winit::event::VirtualKeyCode::Equals => keyboard::Key::Equal,

                winit::event::VirtualKeyCode::Home => keyboard::Key::Home,
                winit::event::VirtualKeyCode::End => keyboard::Key::End,
                winit::event::VirtualKeyCode::Insert => keyboard::Key::Insert,
                winit::event::VirtualKeyCode::Delete => keyboard::Key::Delete,
                winit::event::VirtualKeyCode::PageUp => keyboard::Key::PageUp,
                winit::event::VirtualKeyCode::PageDown => keyboard::Key::PageDown,

                winit::event::VirtualKeyCode::Left => keyboard::Key::Arrow{ direction: keyboard::Direction::Left },
                winit::event::VirtualKeyCode::Right => keyboard::Key::Arrow{ direction: keyboard::Direction::Right },
                winit::event::VirtualKeyCode::Up => keyboard::Key::Arrow{ direction: keyboard::Direction::Up },
                winit::event::VirtualKeyCode::Down => keyboard::Key::Arrow{ direction: keyboard::Direction::Down },

                winit::event::VirtualKeyCode::Numpad0 => keyboard::Key::Numpad(0),
                winit::event::VirtualKeyCode::Numpad1 => keyboard::Key::Numpad(1),
                winit::event::VirtualKeyCode::Numpad2 => keyboard::Key::Numpad(2),
                winit::event::VirtualKeyCode::Numpad3 => keyboard::Key::Numpad(3),
                winit::event::VirtualKeyCode::Numpad4 => keyboard::Key::Numpad(4),
                winit::event::VirtualKeyCode::Numpad5 => keyboard::Key::Numpad(5),
                winit::event::VirtualKeyCode::Numpad6 => keyboard::Key::Numpad(6),
                winit::event::VirtualKeyCode::Numpad7 => keyboard::Key::Numpad(7),
                winit::event::VirtualKeyCode::Numpad8 => keyboard::Key::Numpad(8),
                winit::event::VirtualKeyCode::Numpad9 => keyboard::Key::Numpad(9),

                // Those ones are definitively relevant, but they were buggy on my Linux.
                winit::event::VirtualKeyCode::Numlock => keyboard::Key::Unknown(0),//
                winit::event::VirtualKeyCode::NumpadComma => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::NumpadEnter => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::NumpadEquals => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Colon => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Decimal => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Divide => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Add => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Stop => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Minus => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Multiply => keyboard::Key::Unknown(0),

                // The following group probably has some use...
                winit::event::VirtualKeyCode::Snapshot => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Scroll => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Pause => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Compose => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Caret => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Capital => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::LWin => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::RWin => keyboard::Key::Unknown(0),

                // The rest should probably be supported; to be added as the testing goes...
                winit::event::VirtualKeyCode::Calculator => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Mail => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Convert => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Grave => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Kana => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Kanji => keyboard::Key::Unknown(0),

                winit::event::VirtualKeyCode::MediaSelect => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::MediaStop => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::NavigateForward => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::NavigateBackward => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::PlayPause => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::PrevTrack => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::NextTrack => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::VolumeDown => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::VolumeUp => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Mute => keyboard::Key::Unknown(0),

                winit::event::VirtualKeyCode::Power => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Sleep => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Wake => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Yen => keyboard::Key::Unknown(0),

                winit::event::VirtualKeyCode::WebBack => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::WebFavorites => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::WebForward => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::WebHome => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::WebRefresh => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::WebSearch => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::WebStop => keyboard::Key::Unknown(0),

                winit::event::VirtualKeyCode::Copy => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Paste => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Cut => keyboard::Key::Unknown(0),

                // Weird keys... to be added as they're tested.
                winit::event::VirtualKeyCode::AbntC1 => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::AbntC2 => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Apps => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::At => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Ax => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::MyComputer => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::NoConvert => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::OEM102 => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Sysrq => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Underline => keyboard::Key::Unknown(0),
                winit::event::VirtualKeyCode::Unlabeled => keyboard::Key::Unknown(0)
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
    position: Position<i32>,
    pub size: Size<i32>,

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
    pub fn new(size: Size<i32>) -> Window<States> {
        let event_loop = get_or_create_event_loop();
        let shared_context = get_or_create_context();

        let window_builder = winit::window::WindowBuilder::new()
            .with_inner_size(winit::dpi::LogicalSize::new(size.width, size.height));

        let windowed_context = glutin::ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGlEs, (3, 2)))
            .with_gl_profile(GlProfile::Core)
            .with_srgb(false)
            .with_multisampling(0)
            .with_shared_lists(shared_context)
            .build_windowed(window_builder, &event_loop)
            .unwrap();

        let (raw_context, window) = unsafe {
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
    /// The **title() function** is not documented yet. Pull requests are welcome.
    ///
    pub fn title(&self) -> String {
        self.title.clone()
    }

    ///
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
    fn handle_keyboard_input(&mut self, states: &mut States, input: &winit::event::KeyboardInput, _is_synthetic: &bool) {
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
        let vector = Vector::from_xy(0.0, 0.0);

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
                          _states: &mut States,
                          _state: &winit::event::ElementState,
                          _button: &winit::event::MouseButton,
                          _modifiers: &winit::event::ModifiersState) {
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

    pub fn redraw(&mut self) {
        self.window.request_redraw();
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
