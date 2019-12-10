// Copyright (c) 2020 - BytePlug
//
// This source file is part of Distilled Multimedia Library which is released
// under the MIT license. Please refer to the LICENSE file that can be found
// at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

pub mod geometry {
    trait Movable {}
    trait Rotable {}
    trait Scalable {}

    struct Transform {}
    trait Transformable {}

    trait Resizable {}

    struct Vector {}

    struct Point {}
    struct Line {}
    struct Rectangle {}
    struct Circle {}
}