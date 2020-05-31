// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, May 2020

#[derive(Default)]
pub struct Options {
    depth_bits: Option<u32>,
    stencil_bits: Option<u32>,
    antialiasing_level: Option<u32>,
    srgb: Option<u32>,
    vsync: Option<u32>,
    debug: Option<u32>
}
