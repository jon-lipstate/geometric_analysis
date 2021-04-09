#![allow(dead_code, unused_imports)]
use nalgebra::Vector2;
mod convex_hull;
use convex_hull::{graham_scan, jarvis_march};
mod kernel;
use kernel::Angle;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let p: Vector2<f32> = Vector2::new(0f32, 1f32);
    let q: Vector2<f32> = Vector2::new(0f32, 0f32);
    let r: Vector2<f32> = Vector2::new(1f32, 0f32);
    let a = Angle::get_by_points(&p, &q, &r);
    println!("{:?}", a);
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
