#![allow(unused_variables)]

use crate::kernel::primatives::Point2;
use crate::Vector2;

#[derive(PartialEq, Debug)]
pub enum Angle {
    Obtuse,
    Right,
    Acute,
}

impl Angle {
    pub fn get_by_vector(u: &Vector2<f32>, v: &Vector2<f32>) -> Angle {
        return Angle::Right;
    }
    pub fn get_by_points(p: &Point2, q: &Point2, r: &Point2) -> Angle {
        return Angle::Right;
    }
    //todo impl 3d versions...

    pub fn approximate_by_vector(u: &Vector2<f32>, v: &Vector2<f32>) {}
    pub fn approximate_by_points(p: &Point2, q: &Point2, r: &Point2) {}

    pub fn approximate_dihedral(p: &Point2, q: &Point2, r: &Point2, s: &Point2) {}
}
