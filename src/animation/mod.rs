// Copyright (c) 2020 - BytePlug
//
// This source file is part of Distilled Multimedia Library which is released
// under the MIT license. Please refer to the LICENSE file that can be found
// at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

//! Procedural animation functionalities
//!
//! Additional documentation is to be written here.
mod animator;
mod animation;

mod move_animation;
mod rotate_animation;
mod scale_animation;
mod resize_animation;

mod frame_animation;
mod skeleton_animation;

mod timeline;

pub use animator::Animator;
pub use animation::Animation;

pub use move_animation::MoveAnimation;
pub use rotate_animation::RotateAnimation;
pub use scale_animation::ScaleAnimation;
pub use resize_animation::ResizeAnimation;

pub use frame_animation::FrameAnimation;
pub use skeleton_animation::SkeletonAnimation;

pub use timeline::Timeline;
