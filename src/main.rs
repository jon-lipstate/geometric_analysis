#![allow(dead_code, unused_imports)]
use nalgebra::Vector2;
mod convex_hull;
use convex_hull::{graham_scan, jarvis_march};
mod line_segment2;
mod point2;
mod polygon_2d;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let points: Vec<Vector2<f32>> = vec![
        Vector2::new(0f32, 0f32),
        Vector2::new(1f32, 0f32),
        Vector2::new(1f32, 1f32),
        Vector2::new(0f32, 1f32),
        Vector2::new(0f32, 0f32),
        //Clockwise Subtraction
        // Vector2::new(0f32, 5f32),
        // Vector2::new(5f32, 5f32),
        // Vector2::new(5f32, 0f32),
        // Vector2::new(0f32, 0f32),
    ];
    let area = polygon_2d::area(&points);
    let centroid = polygon_2d::centroid(&points);
    //let moi = polygon_2d::moment_of_inertia(&points, Some(Vector2::new(0f32, 0f32)));
    let moi = polygon_2d::moment_of_inertia(&points, None);
    println!("Area: {}, Centroid: {}", area, centroid);
    println!("moi: {:?}", moi);
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
