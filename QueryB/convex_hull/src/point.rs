use crate::convex_hull::*;

#[derive(Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
