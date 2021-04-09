// use nalgebra::Vector2;
// pub struct Point2 {
//     pub origin: &'static Point2,
//     pub x: f32,
//     pub y: f32,
//     //pub dimension: Dimension (units?)
// }
// impl Point2 {
//     pub fn new(x: f32, y: f32, origin: &'static Point2) -> Self {
//         Self { x, y, origin }
//     }
//     pub fn to_vector2(&self) -> Vector2<f32> {
//         Vector2::new(self.x, self.y)
//     }
// }

use nalgebra::Vector2 as v2;

pub type Point2 = v2<f32>;
