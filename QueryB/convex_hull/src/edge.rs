use crate::functions::*;
use crate::point::Point;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edge {
    pub point1: Point,
    pub point2: Point,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.point1 == other.point1 && self.point2 == other.point2) || (self.point1 == other.point2 && self.point2 == other.point1)
    }
}
