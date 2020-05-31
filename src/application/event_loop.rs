// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, May 2020

use winit::event_loop::EventLoop;

static mut EVENT_LOOP: Option<EventLoop<()>> = None;

fn ensure_event_loop() {
    unsafe {
        // todo: there must be a more elegant way to 'initialize only if the content is None
        match &EVENT_LOOP {
            Some(_) => (),
            None => {
                let event_loop = EventLoop::new();
                EVENT_LOOP = Some(event_loop);
            }
        }
    }
}

pub fn get_or_create_event_loop() -> &'static mut EventLoop<()> {
    ensure_event_loop();

    unsafe {
        EVENT_LOOP.as_mut().unwrap()
    }
}
