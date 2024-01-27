use crate::{functions::*, plane};
use crate::point::Point;
use crate::plane::Plane;
use crate::edge::Edge;

#[derive(Clone, Debug)]
pub struct ConvexHull {
    pub points: Vec<Point>,
    pub edges: Vec<Edge>,
    pub planes: Vec<Plane>,
}

impl ConvexHull {
    pub fn new(points: Vec<Point>) -> ConvexHull {
        ConvexHull {
            points: points,
            edges: Vec::new(),
            planes: Vec::new(),
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Point> {
        self.points.iter()
    }

    // a function to initialize the convex hull with the first four points
    pub fn initialize(&mut self) {
        // create the first edges
        let edge1 = Edge {
            point1: self.points[0].clone(),
            point2: self.points[1].clone(),
        };
        let edge2 = Edge {
            point1: self.points[1].clone(),
            point2: self.points[2].clone(),
        };
        let edge3 = Edge {
            point1: self.points[2].clone(),
            point2: self.points[0].clone(),
        };

        let plane = Plane {
            point_a: self.points[0].clone(),
            point_b: self.points[1].clone(),
            point_c: self.points[2].clone(),
            normal: Point { x: 0, y: 0, z: 0 },
            edge_a: edge1.clone(),
            edge_b: edge2.clone(),
            edge_c: edge3.clone(),
        };

        // add the edges to the edges vector
        self.edges.push(edge1);
        self.edges.push(edge2);
        self.edges.push(edge3);

        self.add_plane(plane);
        // println!("Point is above plane");
        let temp_points = self.planes[0].get_points();
        for i in 0..3 {
            let edge = Edge {
                point1: temp_points[i].clone(),
                point2: self.points[3].clone(),
            };
            self.edges.push(edge);
        }
        for i in 0..3 {
            let plane = Plane {
                point_a: temp_points[i].clone(),
                point_b: temp_points[(i + 1) % 3].clone(),
                point_c: self.points[3].clone(),
                normal: Point { x: 0, y: 0, z: 0 },
                edge_a: Edge {
                    point1: temp_points[i].clone(),
                    point2: temp_points[(i + 1) % 3].clone(),
                },
                edge_b: Edge {
                    point1: temp_points[(i + 1) % 3].clone(),
                    point2: self.points[3].clone(),
                },
                edge_c: Edge {
                    point1: self.points[3].clone(),
                    point2: temp_points[i].clone(),
                },
            };
            // for each plane add the edges to the edges vector
            self.add_plane(plane);
        }


    }

    pub fn add_plane(&mut self, plane: Plane) {
        let mut plane = plane;
        plane.calculate_normal();
        self.planes.push(plane);
    }

    pub fn remove_plane(&mut self, plane: Plane) {
        self.planes.remove(self.planes.iter().position(|p| *p == plane).unwrap());
    }

    pub fn add_point(&mut self, point: Point, plane: Plane) {
        let temp_points = plane.get_points();
        // craete new edges
        for i in 0..3 {
            let edge = Edge {
                point1: temp_points[i].clone(),
                point2: point.clone(),
            };
            // add new edges to convex hull
            self.edges.push(edge);
        }
        // create new planes
        for i in 0..3 {
            let plane = Plane {
                point_a: temp_points[i].clone(),
                point_b: temp_points[(i + 1) % 3].clone(),
                point_c: point.clone(),
                normal: Point { x: 0, y: 0, z: 0 },
                edge_a: Edge {
                    point1: temp_points[i].clone(),
                    point2: temp_points[(i + 1) % 3].clone(),
                },
                edge_b: Edge {
                    point1: temp_points[(i + 1) % 3].clone(),
                    point2: point.clone(),
                },
                edge_c: Edge {
                    point1: point.clone(),
                    point2: temp_points[i].clone(),
                },
            };
            self.add_plane(plane);
        }

        // remove the plane from the planes vector
        self.remove_plane(plane.clone());

        // add the point to the points vector
        self.points.push(point.clone());
    }
    
}
