use crate::kernel::{
    cmp_f32,
    primatives::{Point2, Vector2},
};
use std::cmp::Ordering;
#[derive(PartialEq, Debug)]
pub enum Orientation {
    Collinear,
    Clockwise,
    CounterClockwise,
}

impl Orientation {
    pub fn get(a: &Point2, b: &Point2, c: &Point2) -> Orientation {
        let det: f32 = (b.x - a.x) * (c.y - b.y) - (b.y - a.y) * (c.x - b.x);

        if cmp_f32(0f32, det) == Ordering::Equal {
            return Orientation::Collinear;
        } else if det > 0f32 {
            return Orientation::CounterClockwise;
        } else {
            return Orientation::Clockwise;
        }
    }
}
