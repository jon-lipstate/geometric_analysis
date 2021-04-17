use super::super::Orientation;
use super::Point2;
pub struct Segment2 {
    pub start: Point2,
    pub end: Point2,
}
impl Segment2 {
    pub fn new(start: Point2, end: Point2) -> Self {
        Self { start, end }
    }
    pub fn intersects(&self, other: &Segment2) -> bool {
        //https://www.geeksforgeeks.org/check-if-two-given-line-segments-intersect/
        let self_other_start = Orientation::get(&self.start, &self.end, &other.start);
        let self_other_end = Orientation::get(&self.start, &self.end, &other.end);
        let other_self_start = Orientation::get(&other.start, &other.end, &self.start);
        let other_self_end = Orientation::get(&other.start, &other.end, &self.end);
        //General Case:
        if self_other_start != self_other_end && other_self_start != other_self_end {
            return true;
        }

        // Special Cases - Colinearity
        if self_other_start == Orientation::Collinear && self.on_segment(&other.start) {
            return true;
        } else if self_other_end == Orientation::Collinear && self.on_segment(&other.end) {
            return true;
        } else if other_self_start == Orientation::Collinear && other.on_segment(&self.start) {
            return true;
        } else if other_self_end == Orientation::Collinear && other.on_segment(&self.end) {
            return true;
        }

        false
    }

    pub fn on_segment(&self, other: &Point2) -> bool {
        match Orientation::get(&self.start, &self.end, other) {
            Orientation::Collinear => {
                let x_lt = self.end.x <= f32::max(self.start.x, other.x);
                let x_gt = self.end.x >= f32::min(self.start.x, other.x);

                let y_lt = self.end.y <= f32::max(self.start.y, other.y);
                let y_gt = self.end.y >= f32::min(self.start.y, other.y);

                if x_lt && x_gt && y_lt && y_gt {
                    return true;
                }
                false
            }
            _ => false,
        }
    }
}
