use nalgebra::Vector2;
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}
impl Point2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn to_vector2(&self) -> Vector2<f32> {
        Vector2::new(self.x, self.y)
    }
}
