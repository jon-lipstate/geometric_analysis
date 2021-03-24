#![allow(dead_code)]
use nalgebra::Vector2;
mod graham_scan;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut points: Vec<Vector2<f32>> = vec![
        Vector2::new(0f32, 0f32),
        Vector2::new(10f32, 0f32),
        Vector2::new(10f32, 10f32),
        Vector2::new(0f32, 10f32),
        Vector2::new(3f32, 0f32),
        Vector2::new(1f32, 0f32),
        Vector2::new(3f32, 3f32),
    ];
    let hull = graham_scan::graham_scan(&mut points);
    // let hull = graham_scan::g_scan(&mut points);
    // let h = ConvexHull::new(points);
    println!("Hull Length: {}", hull.len());
    let vv: Vec<(f32, f32)> = hull.iter().map(|x: &Vector2<f32>| (x.x, x.y)).collect();
    println!("Hull: {:?}", vv);
    Ok(())
}
//concave hull:
// Find the point with the lowest y (latitude) coordinate and make it the current one.
// Find the k-nearest points to the current point.
// From the k-nearest points, select the one which corresponds to the largest right-hand turn from the previous angle. Here we will use the concept of bearing and start with an angle of 270 degrees (due West).
// Check if by adding the new point to the growing line string, it does not intersect itself. If it does, select another point from the k-nearest or restart with a larger value of k.
// Make the new point the current point and remove it from the list.
// After k iterations add the first point back to the list.
// Loop to number 2.

pub struct ConvexHull {
    hull: Vec<Vector2<f32>>,
    raw: Vec<Vector2<f32>>,
}
impl ConvexHull {
    //returns list of clockwise points of convex hull
    pub fn new(points: Vec<Vector2<f32>>) -> Self {
        //Assert CCW Angularity
        //Assert n>3
        if points.len() < 3 {
            panic!(
                "Convex Hull Requires 3 points or more, given: {:?} points",
                points.len()
            );
        }
        //Sort to X-Min
        points
            .to_owned()
            .sort_unstable_by(|a: &Vector2<f32>, b: &Vector2<f32>| {
                (b.x).partial_cmp(&a.x).unwrap()
            });

        let mut upper: Vec<Vector2<f32>> = vec![points[0], points[1]];

        for i in 2..points.len() {
            upper.push(points[i]);
            // println!("U: i: {}, point i: {}", i, points[i]);
            let is_ccw = is_ccw(
                upper[upper.len() - 3],
                upper[upper.len() - 2],
                upper[upper.len() - 1],
            );
            while upper.len() > 2 && is_ccw {
                upper.remove(upper.len() - 2); //Delete middle of last 3 points
            }
        }

        let mut lower: Vec<Vector2<f32>> = vec![points[points.len() - 1], points[points.len() - 2]];

        for i in (0..points.len() - 3).rev() {
            // println!("LOWERRRRRR: i: {}, point i: {}", i, points[i]);
            lower.push(points[i]);
            let is_ccw = is_ccw(
                lower[lower.len() - 3],
                lower[lower.len() - 2],
                lower[lower.len() - 1],
            );
            while lower.len() > 2 && is_ccw {
                lower.remove(lower.len() - 2); //Delete middle of last 3 points
            }
        }
        //remove first and last element to not clash with upper:
        lower.remove(lower.len() - 1);
        lower.remove(0);

        upper.append(&mut lower);

        Self {
            hull: upper,
            raw: points,
        }
    }
}

pub fn is_ccw(p0: Vector2<f32>, p1: Vector2<f32>, p2: Vector2<f32>) -> bool {
    let a: Vector2<f32> = p1 - p0;
    let b: Vector2<f32> = p2 - p1;
    let det = a.x * b.y - a.y * b.x;
    let dot = (a.x * b.x + a.y * b.y) / (a.magnitude() * b.magnitude());
    let angle = f32::atan2(det, dot);
    return angle < 0f32;
}
