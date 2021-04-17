#![allow(unused_variables)]

use crate::kernel::{Point2, Vector2};

#[derive(PartialEq, Debug)]
pub enum Angle {
    Obtuse,
    Right,
    Acute,
}

impl Angle {
    pub fn get_by_vector(u: &Vector2, v: &Vector2) -> Angle {
        return Angle::Right;
    }
    pub fn get_by_points(p: &Point2, q: &Point2, r: &Point2) -> Angle {
        return Angle::Right;
    }
    //todo impl 3d versions...

    pub fn approximate_by_vector(u: &Vector2, v: &Vector2) {}
    pub fn approximate_by_points(p: &Point2, q: &Point2, r: &Point2) {}
    pub fn approximate_dihedral(p: &Point2, q: &Point2, r: &Point2, s: &Point2) {}
}
