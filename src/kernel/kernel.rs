use crate::kernel::primatives::{Point2, Vector2};
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

///TODO: Allow any primative, a point/line would do the normal distance etc
pub fn dist_squared(a: &Vector2, b: &Vector2) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    return dx * dx + dy * dy;
}

//Predicate: discrete set of results (e.g. orientation)
//Construction: Produces a # or geo entity, e.g. distance
