use nalgebra::Vector2;
use std::cmp::Ordering;

pub fn cmp_f32(a: f32, b: f32) -> Ordering {
    let threshold = 2f32 * f32::EPSILON;
    let diff = b - a;
    let is_zero = diff + threshold > 0f32 && diff - threshold < 0f32;
    if is_zero {
        return Ordering::Equal;
    } else if diff > 0f32 {
        return Ordering::Greater;
    } else {
        return Ordering::Less;
    }
}

pub fn get_orientation(a: &Vector2<f32>, b: &Vector2<f32>, c: &Vector2<f32>) -> Orientation {
    let det: f32 = (b.x - a.x) * (c.y - b.y) - (b.y - a.y) * (c.x - b.x);

    if cmp_f32(0f32, det) == Ordering::Equal {
        return Orientation::Collinear;
    } else if det > 0f32 {
        return Orientation::CounterClockwise;
    } else {
        return Orientation::Clockwise;
    }
}
#[derive(PartialEq, Debug)]
pub enum Orientation {
    Collinear,
    Clockwise,
    CounterClockwise,
}
