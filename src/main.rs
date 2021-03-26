#![allow(dead_code, unused_imports)]
use nalgebra::Vector2;
mod convex_hull;
use convex_hull::{graham_scan, jarvis_march};

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

    let hull = graham_scan(&mut points.clone());
    println!("graham_scan, Hull Length: {}", hull.len());
    let vv: Vec<(f32, f32)> = hull.iter().map(|x: &Vector2<f32>| (x.x, x.y)).collect();
    println!("Hull: {:?}", vv);

    let hull = jarvis_march(&mut points);
    println!("jarvis_march, Hull Length: {}", hull.len());
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
