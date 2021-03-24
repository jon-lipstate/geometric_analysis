use nalgebra::Vector2;
use std::cmp::Ordering;
//https://www.geeksforgeeks.org/convex-hull-set-2-graham-scan/
//https://www.youtube.com/watch?v=vPDPE66nhlo

fn get_orientation(p: &Vector2<f32>, q: &Vector2<f32>, r: &Vector2<f32>) -> Orientation {
    let det: f32 = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
    let threshold = 2f32 * f32::EPSILON;
    let is_zero = det + threshold > 0f32 && det - threshold < 0f32;
    if is_zero {
        return Orientation::Collinear;
    } else if det > 0f32 {
        return Orientation::Clockwise;
    } else {
        return Orientation::CounterClockwise;
    }
}
#[derive(PartialEq, Debug)]
enum Orientation {
    Collinear,
    Clockwise,
    CounterClockwise,
}

fn polar_angle(p: &Vector2<f32>, anchor: &Vector2<f32>) -> f32 {
    //q is anchor
    let x_span = p.x - anchor.x;
    let y_span = p.y - anchor.y;
    f32::atan2(y_span, x_span)
}

pub fn graham_scan(points: &mut Vec<Vector2<f32>>) -> Vec<Vector2<f32>> {
    //Find Min Y Point & Set as Anchor:
    let mut y_min = points[0].y;
    let mut min_index = 0;
    for i in 1..points.len() {
        let y = points[i].y;
        if y < y_min || (y == y_min && points[i].x < points[min_index].x) {
            y_min = points[i].y;
            min_index = i;
        }
    }
    let anchor: Vector2<f32> = points[min_index];
    points.swap(0, min_index);
    //Sort Relative to Anchor:
    points.sort_by(|a: &Vector2<f32>, b: &Vector2<f32>| {
        let angle_a = polar_angle(a, &anchor);
        let angle_b = polar_angle(b, &anchor);
        if angle_a == angle_b {
            let da = f32::powi(anchor.x - a.x, 2) + f32::powi(anchor.y - a.y, 2);
            let db = f32::powi(anchor.x - b.x, 2) + f32::powi(anchor.y - b.y, 2);
            if da > db {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        } else if angle_a > angle_b {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    });
    //Perform the Scan:
    let mut hull: Vec<Vector2<f32>> = vec![anchor, points[1]];
    for i in 2..points.len() {
        loop {
            let last: &Vector2<f32> = &hull[hull.len() - 1];
            let next_last: &Vector2<f32> = &hull[hull.len() - 2];
            let next: &Vector2<f32> = &points[i];

            if get_orientation(next_last, last, next) == Orientation::CounterClockwise {
                break;
            }
            hull.remove(hull.len() - 1);

            if hull.len() < 2 {
                break;
            }
        } //end loop
        hull.push(points[i]);
    } // end for

    return hull;
}
