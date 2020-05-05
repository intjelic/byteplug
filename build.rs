// Copyright (c) 2020 - BytePlug
//
// This source file is part of Distilled Multimedia Library which is released
// under the MIT license. Please refer to the LICENSE file that can be found
// at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

extern crate gl_generator;

use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let destination = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&destination).join("bindings.rs")).unwrap();

    Registry::new(Api::Gles2, (3, 2), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();
}
