// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of the Byteplug framework which is released under the MIT license.
// Please refer to the LICENSE file that can be found at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, June 2020

use std::ops::Sub;
use crate::geometry::{Position, Size};
use crate::geometry::Box;

pub fn compute_bounds<T>(points: &Vec<Position<T>>) -> Box<T>
where T: Sub<Output = T> + PartialOrd + Default + Copy
{
    if points.is_empty() {
        return Box::default()
    }

    let mut left   = points[0].x;
    let mut top    = points[0].y;
    let mut right  = points[0].x;
    let mut bottom = points[0].y;

    for point in points.iter() {
        if point.x < left {
            left = point.x;
        }
        else if point.x > right {
            right = point.x;
        }

        if point.y < top {
            top = point.y;
        }
        else if point.y > bottom {
            bottom = point.y;
        }
    };

    Box::new(Position::new(left, top), Size::new(right - left, bottom - top))
}
