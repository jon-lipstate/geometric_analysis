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

        let lace = -(current.x * next.y - next.x * current.y) / 2f32; //max reverses this..?

        ixo += (f32::powi(current.y, 2) + current.y * next.y + f32::powi(next.y, 2)) / 6f32 * lace;
        iyo += lace * (f32::powi(current.x, 2) + current.x * next.x + f32::powi(next.x, 2)) / 6f32;
        ixyo += lace
            * (current.x * (2f32 * current.y + next.y) + next.x * (2f32 * next.y + current.y))
            / 12f32;

        println!(
            "lace: {}, cy2: {},cyny: {}, ny2: {}, (...) {}, lace/6: {}",
            lace,
            f32::powi(current.y, 2),
            current.y * next.y,
            f32::powi(next.y, 2),
            f32::powi(current.y, 2) + current.y * next.y + f32::powi(next.y, 2),
            lace / 6f32
        );

        println!(
            "ixo: {}, c:{},{}, n:{},{}",
            ixo, current.x, current.y, next.x, next.y
        );
    }

    return (ixo, iyo, ixyo);
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

//https://github.com/cmccomb/structural-shapes
//https://github.com/suavecode/SUAVE
