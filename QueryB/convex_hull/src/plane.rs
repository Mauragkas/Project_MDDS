use crate::functions::*;
use crate::point::Point;
use crate::edge::Edge;

#[derive(Clone, Debug)]
pub struct Plane {
    pub point_a: Point,
    pub point_b: Point,
    pub point_c: Point,

    pub normal: Point, // its normal vector is calculated from the three points
 
    pub edge_a: Edge,
    pub edge_b: Edge,
    pub edge_c: Edge,
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        (self.point_a == other.point_a && self.point_b == other.point_b && self.point_c == other.point_c) || (self.point_a == other.point_a && self.point_b == other.point_c && self.point_c == other.point_b) || (self.point_a == other.point_b && self.point_b == other.point_a && self.point_c == other.point_c) || (self.point_a == other.point_b && self.point_b == other.point_c && self.point_c == other.point_a) || (self.point_a == other.point_c && self.point_b == other.point_a && self.point_c == other.point_b) || (self.point_a == other.point_c && self.point_b == other.point_b && self.point_c == other.point_a)
    }
}

impl Plane {
    pub fn calculate_normal(&mut self) {
        let vector1 = subtract_vectors(self.point_b.clone(), self.point_a.clone());
        let vector2 = subtract_vectors(self.point_c.clone(), self.point_a.clone());
        self.normal = cross_product(vector1, vector2);
    }

    pub fn get_points(&self) -> Vec<Point> {
        vec![self.point_a.clone(), self.point_b.clone(), self.point_c.clone()]
    }
}