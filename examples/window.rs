
/*
   This example is part of the Byteplug framework.

   It shows how to write a typical one window application that is not cross-platform (see the
   application.rs example for that). It's not meant to be a minimal example, but instead showcases
   the concepts directly related to working with a window (such as its inner surface and handling
   input events). Nothing is displayed, only a background made of a solid color computed according
   to the position of the mouse.

   Things left to do
   -----------------
   - This example may drastically change because the interface of the widget module is not entirely
     decided.

   For more question, visit https://www.byteplug.io/framework.
*/

#[link(name="GLESv2")] extern {}

use byteplug::geometry::{Position, Size};
use byteplug::image::Color;
use byteplug::application::Window;

fn compute_color(position: Position<i32>, size: Size<i32>) -> Color {
    // Compute the background color based on the position of the cursor and the size of the window;
    // the X axis defines the RGB color.
    let u = position.x as f32 / size.width as f32;

    let (r, g, b) = if u < (1.0 / 3.0) {
        (u / (1.0 / 3.0), 0.0, 0.0)
    }
    else if u < (2.0 / 3.0) {
        (0.0, (u - (1.0 / 3.0)) / (1.0 / 3.0), 0.0)
    }
    else {
        (0.0, 0.0, (u - (2.0 / 3.0)) / (1.0 / 3.0))
    };

    Color::rgb(
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8
    )
}

// States of the application; a unique instance of this structure is shared and passed down to the
// main components.
struct States {
}

fn main() {
    // Create the application states and initialize them.
    let mut states = States {
    };

    // Create a window and initialize it (setting up of window-related callbacks; they all receive
    // the application states).
    let mut window = Window::new(Size::new(640, 480));

    window.on_move = Some(|_window: &mut Window<States>, position, _states| {
        println!("on_move({:?})", position);
    });
    window.on_resize = Some(|window, size, _states| {
        println!("on_resize({:?})", size);
        window.surface().resize(size);
    });
    window.on_draw = Some(|window, _pixels, _states| {
        println!("on_draw()");

        window.surface().swap();
    });

    window.on_focus_gain = Some(|_window, _states| {
        println!("on_focus_gain()");
    });
    window.on_focus_lose = Some(|_window, _states| {
        println!("on_focus_lose()");
    });

    window.on_key_down = Some(|_window, key, modifiers, _states| {
        println!("on_key_down({:?}, {:?})", key, modifiers);
    });
    window.on_key_up = Some(|_window, key, modifiers, _states| {
        println!("on_key_up({:?}, {:?})", key, modifiers);
    });
    window.on_character_enter = Some(|_window, character, _states| {
        println!("on_character_enter({:?})", character);
    });

    window.on_cursor_enter = Some(|_window, position, _states| {
        println!("on_cursor_enter({:?})", position);
    });
    window.on_cursor_leave = Some(|_window, position, _states| {
        println!("on_cursor_leave({:?})", position);
    });
    window.on_cursor_move = Some(|window, position, movement, _states| {
        println!("on_cursor_move({:?}, {:?})", position, movement);

        let color = compute_color(position, window.size);
        window.surface().erase(color);
        window.redraw();
    });

    window.on_mouse_down = Some(|_window, button, position, _states| {
        println!("on_mouse_down({:?}, {:?})", button, position);
    });
    window.on_mouse_up = Some(|_window, button, position, _states| {
        println!("on_mouse_up({:?}, {:?})", button, position);
    });
    window.on_mouse_scroll = Some(|_window, wheel, movement, _states| {
        println!("on_mouse_scroll({:?}, {:?})", wheel, movement);
    });

    // Start the main loop; we pass the application states which will be passed down to all window
    // callback functions.
    window.run(&mut states);

    // Release the application states here -- In this case, nothing to do...
}
