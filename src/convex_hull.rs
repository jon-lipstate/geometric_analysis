use crate::kernel::{cmp_f32, dist_squared, Orientation};
use nalgebra::Vector2;
use std::cmp::Ordering;
//https://www.geeksforgeeks.org/convex-hull-set-2-graham-scan/
//https://www.youtube.com/watch?v=vPDPE66nhlo

//Sample Main Code:
// use nalgebra::Vector2;
// mod convex_hull;
// use convex_hull::{graham_scan, jarvis_march};

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mut points: Vec<Vector2<f32>> = vec![
//         Vector2::new(0f32, 0f32),
//         Vector2::new(10f32, 0f32),
//         Vector2::new(10f32, 10f32),
//         Vector2::new(0f32, 10f32),
//         Vector2::new(3f32, 0f32),
//         Vector2::new(1f32, 0f32),
//         Vector2::new(3f32, 3f32),
//     ];

//     let hull = graham_scan(&mut points.clone());
//     println!("graham_scan, Hull Length: {}", hull.len());
//     let vv: Vec<(f32, f32)> = hull.iter().map(|x: &Vector2<f32>| (x.x, x.y)).collect();
//     println!("Hull: {:?}", vv);

//     let hull = jarvis_march(&mut points);
//     println!("jarvis_march, Hull Length: {}", hull.len());
//     let vv: Vec<(f32, f32)> = hull.iter().map(|x: &Vector2<f32>| (x.x, x.y)).collect();
//     println!("Hull: {:?}", vv);
//     Ok(())
// }

fn sort_polar(anchor: &Vector2<f32>, a: &Vector2<f32>, b: &Vector2<f32>) -> Ordering {
    let angle_a = f32::atan2(a.x - anchor.x, a.y - anchor.y);
    let angle_b = f32::atan2(b.x - anchor.x, b.y - anchor.y);

    match cmp_f32(angle_a, angle_b) {
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
        Ordering::Equal => {
            //Collinear: if Slope is Positive, Sort Ascending, Slope is neg or Vertical, sort Descending
            let b_is_farther = cmp_f32(dist_squared(anchor, b), dist_squared(anchor, a));
            let dx_a = a.x - anchor.x;
            //Vertical Alignment | Negative Slope
            if cmp_f32(0f32, dx_a) == Ordering::Equal || dx_a < 0f32 {
                b_is_farther.reverse()
            } else {
                b_is_farther
            }
        }
    }
}

fn set_bottom_left_point(points: &mut Vec<Vector2<f32>>) {
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
    points.swap(0, min_index);
}

///O(nlogn)
//https://youtu.be/B2AJoQSZf4M
pub fn graham_scan(points: &mut Vec<Vector2<f32>>) -> Vec<Vector2<f32>> {
    //1. Pick Bottom Left Point (anchor)
    //2. Sort all other points relative (by angle) to anchor
    //2a. X-Product - save cpu cycles, VxW -> +:CCW, -:CW, 0:||
    //3. Push Points onto a stack, if CCW, continue, if CW, pop until CCW
    set_bottom_left_point(points);
    let anchor: Vector2<f32> = points[0];
    points.remove(0);
    //Sort Relative to Anchor:
    points.sort_by(|a: &Vector2<f32>, b: &Vector2<f32>| sort_polar(&anchor, a, b));
    let vv: Vec<(f32, f32)> = points.iter().map(|x: &Vector2<f32>| (x.x, x.y)).collect();
    println!("sorted points: {:?}", vv);
    //Perform the Scan:
    let mut hull: Vec<Vector2<f32>> = vec![anchor, points[0]];
    for i in 1..points.len() {
        loop {
            let hull_last: &Vector2<f32> = &hull[hull.len() - 1];
            let hull_second_last: &Vector2<f32> = &hull[hull.len() - 2];
            let points_next: &Vector2<f32> = &points[i];

            if Orientation::get(hull_second_last, hull_last, points_next)
                == Orientation::CounterClockwise
            {
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

///O(nh)
//https://youtu.be/gfZUkFf3kVA
pub fn jarvis_march(points: &mut Vec<Vector2<f32>>) -> Vec<Vector2<f32>> {
    set_bottom_left_point(points);
    let mut hull: Vec<Vector2<f32>> = vec![points[0]];
    let mut prev: Vector2<f32> = points[0];
    loop {
        let mut candidate: Option<Vector2<f32>> = None;
        for i in 0..points.len() {
            let next: Vector2<f32> = points[i];
            if next == prev {
                continue;
            }
            if candidate.is_none() {
                candidate = Some(next);
                continue;
            }
            match Orientation::get(&prev, &candidate.unwrap(), &next) {
                Orientation::Collinear => {
                    let candidate_is_closer =
                        dist_squared(&prev, &candidate.unwrap()) < dist_squared(&prev, &next);
                    if candidate_is_closer {
                        candidate = Some(next);
                    }
                }
                Orientation::Clockwise => candidate = Some(next),
                Orientation::CounterClockwise => (),
            }
        } // end for
        if candidate.is_some() {
            if candidate.unwrap() == hull[0] {
                //returned to start
                break;
            }
            hull.push(candidate.unwrap());
            prev = candidate.unwrap();
        }
    }

    return hull;
}

/// Takes an unordered hull, and sorts it into CCW Order
pub fn sort_hull() {
    panic!("not implemented");
    // def PolygonSort(corners):
    // # calculate centroid of the polygon
    // n = len(corners) # of corners
    // cx = float(sum(x for x, y in corners)) / n
    // cy = float(sum(y for x, y in corners)) / n
    // # create a new list of corners which includes angles
    // cornersWithAngles = []
    // for x, y in corners:
    //     dx = x - cx
    //     dy = y - cy
    //     an = (math.atan2(dy, dx) + 2.0 * math.pi) % (2.0 * math.pi)
    //     cornersWithAngles.append((dx, dy, an))
    // # sort it using the angles
    // cornersWithAngles.sort(key = lambda tup: tup[2])
    // return cornersWithAngles
}
