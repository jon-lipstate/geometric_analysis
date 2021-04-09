// let points: Vec<Vector2<f32>> = vec![
//         Vector2::new(0f32, 0f32),
//         Vector2::new(1f32, 0f32),
//         Vector2::new(1f32, 1f32),
//         Vector2::new(0f32, 1f32),
//         Vector2::new(0f32, 0f32),
//         //Clockwise Subtraction
//         // Vector2::new(0f32, 5f32),
//         // Vector2::new(5f32, 5f32),
//         // Vector2::new(5f32, 0f32),
//         // Vector2::new(0f32, 0f32),
//     ];
//     // let area = polygon_2d::area(&points);
//     // let centroid = polygon_2d::centroid(&points);
//     // //let moi = polygon_2d::moment_of_inertia(&points, Some(Vector2::new(0f32, 0f32)));
//     // let moi = polygon_2d::moment_of_inertia(&points, None);
//     // println!("Area: {}, Centroid: {}", area, centroid);
//     // println!("moi: {:?}", moi);

use crate::{
    convex_hull::{get_orientation, Orientation},
    kernel::Segment2,
}; //TODO: Move these to a Utilities Module
use nalgebra::Vector2;
/// Ordered List of points representing a manifold polygon
/// Loop Directions -  CCW: Additive, CW: Subtractive
/// Does not require Loop Closure (e.g. first and last points are the same)
/// O(n)
pub fn area(points: &Vec<Vector2<f32>>) -> f32 {
    let first: Vector2<f32> = points[0];
    let last: Vector2<f32> = points[points.len() - 1];

    let mut sum: f32 = last.x * first.y - (first.x * last.y);

    for i in 0..points.len() - 1 {
        let current: Vector2<f32> = points[i];
        let next: Vector2<f32> = points[i + 1];

        sum += current.x * next.y - (next.x * current.y);
    }

    f32::abs(sum / 2f32)
}

///Sums the length of vertices defining the polygon
pub fn perimeter(points: &Vec<Vector2<f32>>) -> f32 {
    let mut perimeter = (points[0] - points[points.len()]).magnitude();
    for i in 0..points.len() - 1 {
        perimeter += (points[i + 1] - points[i]).magnitude();
    }
    return perimeter;
}

pub fn centroid(points: &Vec<Vector2<f32>>) -> Vector2<f32> {
    //derived from:
    //https://github.com/zebengberg/wasatch/blob/ef95a6a090d79d93e26aaa68ab7319a630e18a7a/javascript/canvas_animations/spin.js
    let mut x = 0f32;
    let mut y = 0f32;
    let area_6x = 6f32 * area(points);

    for i in 0..points.len() - 1 {
        let current: Vector2<f32> = points[i];
        let next: Vector2<f32> = points[i + 1];

        let lace = current.x * next.y - next.x * current.y;
        x += (current.x + next.x) * lace;
        y += (current.y + next.y) * lace;
    }

    Vector2::new(x / area_6x, y / area_6x)
}
/// # moment_of_inertia
/// ### Arguments:
/// `points`: Hull of the polygon to be assessed
///
/// `summation_point`: Arbitrary point to calculate MOI about. If providing None, defaults to C.G.
///
/// ### Returns:
/// `(IXX, IYY, IXY)`: Tuple
///
/// ### Assumptions:
/// Hull:
/// - Manifold
/// - Convex or Concave
/// - Ordered Counter-Clockwise
///     - Clockwise ordering produces correct magnitude, but inverted sign.
/// ### References:
/// https://en.wikipedia.org/wiki/Second_moment_of_area#Any_cross_section_defined_as_polygon
///Returns Tuple: (IXX, IYY, IXY)
pub fn moment_of_inertia(
    points: &Vec<Vector2<f32>>,
    summation_point: Option<Vector2<f32>>,
) -> (f32, f32, f32) {
    let sum_pt: Vector2<f32> = summation_point.unwrap_or(centroid(points));
    let is_closed_loop: bool =
        (points[0] - points[points.len() - 1]).magnitude() < f32::EPSILON * 5f32;

    let mut points_sum_pt: Vec<Vector2<f32>> =
        points.iter().map(|p: &Vector2<f32>| p - sum_pt).collect();

    if !is_closed_loop {
        points_sum_pt.push(points_sum_pt[0]);
    }

    let mut ixo = 0f32;
    let mut iyo = 0f32;
    let mut ixyo = 0f32;

    for i in 0..points.len() - 1 {
        let current: Vector2<f32> = points_sum_pt[i];
        let next: Vector2<f32> = points_sum_pt[i + 1];

        let lace = current.x * next.y - next.x * current.y;
        let _ixo = f32::powi(current.y, 2) + current.y * next.y + f32::powi(next.y, 2);
        let _iyo = f32::powi(current.x, 2) + current.x * next.x + f32::powi(next.x, 2);
        let _ixyo = current.x * next.y + 2f32 * current.x * current.y + next.x * current.y;

        ixo += lace * _ixo / 12f32;
        iyo += lace * _iyo / 12f32;
        ixyo += lace * _ixo / 24f32;
    }

    (ixo, iyo, ixyo)
}

pub fn principal_moment_of_inertia(ixx: f32, iyy: f32, ixy: f32) -> (f32, f32, f32) {
    let outside = (ixx + iyy) / 2f32;
    let root = f32::sqrt(f32::powi((ixx - iyy) / 2f32, 2) + f32::powi(ixy, 2));

    let angle = f32::atan(-2f32 * ixy / (ixx - iyy)) / 2f32;
    let prin1 = outside + root;
    let prin2 = outside - root;

    (prin1, prin2, angle)
}

//https://en.wikipedia.org/wiki/First_moment_of_area
// Sum(yA)
fn first_moment_q() {}

fn split_hull(points: &Vec<Vector2<f32>>, split_line: Segment2) {
    let mut intersections: Vec<(usize, Vector2<f32>)> = Vec::new(); //Collect (Index, Direction-Vector)
    for i in 0..points.len() - 1 {
        let segment = Segment2::new(points[i], points[i + 1]);
        if split_line.intersects(&segment) {
            intersections.push((i, &segment.end - &segment.start));
        }
    }
    if intersections.len() % 2 != 0 {
        panic!("Polygon has an un-even number of intersections. Should not be possible.");
    }

    // Test if points[0] is above split-line
    // if so - take points 0-i-1
    // at i, insert an intersection point
    // skip until next i, insert intersection point and so forth

    // Use cross product to determine whether a point lies above or below a line.
    //   Math: https://math.stackexchange.com/a/274728
    //   English: "above" means that looking from point a towards point b,
    //               the point p lies to the left of the line.
    //   is_above = lambda p,a,b: np.cross(p-a, b-a) < 0
}

pub struct Polygon2D {
    pub points: Vec<Vector2<f32>>,
}
