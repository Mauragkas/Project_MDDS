use serde::{Deserialize, Serialize};
use crate::{functions::*, plane};
use crate::point::Point;
use crate::plane::Plane;
use crate::edge::Edge;

#[derive(Clone, Debug, Serialize, Deserialize)]
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

    fn gift_wrapping_init(&mut self) {
        let mut points: Vec<Point> = Vec::new();
        let mut min_z = self.points[0].z;
        // fint the minimum z value
        for point in self.points.iter() {
            if point.z < min_z {
                min_z = point.z;
            }
        }

        // find the points with the minimum z value
        for point in self.points.iter() {
            if point.z == min_z && !points.contains(point) {
                points.push(point.clone());
            }
            if points.len() == 3 {
                break;
            }
        }

        // pop these points from the points vector and push them to the outer points vector
        // for point in points.iter() {
        //     self.points.remove(self.points.iter().position(|p| *p == *point).unwrap());
        //     self.outer_points.push(point.clone());
        // }

        // create the first edges
        let edge1 = Edge::new(points[0].clone(), points[1].clone());

        let edge2 = Edge::new(points[1].clone(), points[2].clone());

        let edge3 = Edge::new(points[2].clone(), points[0].clone());

        // add the edges to the edges vector
        self.edges.push(edge1.clone());
        self.edges.push(edge2.clone());
        self.edges.push(edge3.clone());

        // create the first plane
        let plane = Plane::new(points[0].clone(), points[1].clone(), points[2].clone());

        // add the plane to the planes vector
        self.add_plane(plane);

    }

    // FIXME: This function is not working properly
    pub fn gift_wrapping(&mut self) {
        self.gift_wrapping_init(); // initialize the convex hull with the first plane

        // println!("Edges: {:?}", self.edges);

        // println!("Planes: ");
        // for plane in self.planes.iter() {
        //     println!("\t({:?}, {:?}, {:?})", plane.point_a, plane.point_b, plane.point_c);
        // }

        loop {
            //  find an edge that appears only once in the edges vector
            let mut edges_that_appear_once: Vec<Edge> = Vec::new();
            for edge in self.edges.iter() {
                let mut count = 0;
                for edge2 in self.edges.iter() {
                    if edge == edge2 {
                        count += 1;
                    }
                }
                if count == 1 {
                    edges_that_appear_once.push(edge.clone());
                }
            }

            // println!("Edges that appear once: {:?}", edges_that_appear_once);

            if edges_that_appear_once.len() == 0 {
                break;
            }

            let mut planes_to_add: Vec<Plane> = Vec::new();
            for edge in edges_that_appear_once.iter() {
                // find the plane that contains the edge from the planes vector
                #[allow(non_snake_case)]
                let PLANE = self.planes.iter().find(|p| p.edge_a == *edge || p.edge_b == *edge || p.edge_c == *edge).unwrap();
                // println!("Normal: {:?}", plane.normal);
                let mut possible_planes: Vec<Plane> = Vec::new();
                for point in self.points.iter() {
                    // create a plane from the edge and the point
                    if point != &edge.start && point != &edge.end {
                        let mut plane = Plane::new(edge.start.clone(), edge.end.clone(), point.clone());
                        // push the plane to the possible planes vector
                        possible_planes.push(plane);
                    }
                }

                if possible_planes.contains(&PLANE) {
                    possible_planes.remove(possible_planes.iter().position(|p| *p == PLANE.clone()).unwrap());
                }

                // find the possible plane that has the minimum angle between its normal and the normal of the plane that contains the edge
                let mut min_angle = 360.0;
                let mut min_angle_plane = Plane::new(Point::new(None,0,0,0), Point::new(None,0,0,0), Point::new(None,0,0,0));
                for plane in possible_planes.iter() {
                    let angle = angle_between_vectors(&PLANE.normal, &plane.normal);
                    // println!("Angle: {}", angle);
                    // if angle < min_angle && (angle - 180.0).abs() > 0.0001 {
                    if angle < min_angle && angle > 0.0001 {
                        min_angle = angle;
                        min_angle_plane = plane.clone();
                    }
                }

                planes_to_add.push(min_angle_plane.clone());
            }

            for plane in planes_to_add.iter() {
                self.add_plane(plane.clone());
                let temp_edges = plane.get_edges();
                for edge in temp_edges.iter() {
                    self.edges.push(edge.clone());
                }
                // println!("Plane: ({:?}, {:?}, {:?})", plane.point_a, plane.point_b, plane.point_c);
            }
            // break;
            // println!("Planes now: {}", self.planes.len());
            // for plane in self.planes.iter() {
            //     println!("\t({:?}, {:?}, {:?})", plane.point_a, plane.point_b, plane.point_c);
            // }
        }
        
    }
    
    fn init_simplex(&mut self) {
        // get 4 non collinear points
        let mut points: Vec<Point> = Vec::new();
        match find_non_collinear_points(&self.points) {
            Some(p) => points = p,
            None => println!("No non collinear points found"),
        }

        // create the first 4 planes
        let plane1 = Plane::new(points[1].clone(), points[0].clone(), points[2].clone());
        let plane2 = Plane::new(points[0].clone(), points[1].clone(), points[3].clone());
        let plane3 = Plane::new(points[2].clone(), points[0].clone(), points[3].clone());
        let plane4 = Plane::new(points[1].clone(), points[2].clone(), points[3].clone());

        // add the planes to the planes vector
        self.planes.push(plane1);
        self.planes.push(plane2);
        self.planes.push(plane3);
        self.planes.push(plane4);

        // remove the first 4 points from the points vector
        for point in points.iter() {
            self.points.remove(self.points.iter().position(|p| *p == *point).unwrap());
        }

        // create the first 6 edge
        let edge1 = Edge::new(points[0].clone(), points[1].clone());
        let edge2 = Edge::new(points[0].clone(), points[2].clone());
        let edge3 = Edge::new(points[0].clone(), points[3].clone());
        let edge4 = Edge::new(points[1].clone(), points[2].clone());
        let edge5 = Edge::new(points[1].clone(), points[3].clone());
        let edge6 = Edge::new(points[2].clone(), points[3].clone());

        // add the edges to the edges vector
        self.edges.push(edge1);
        self.edges.push(edge2);
        self.edges.push(edge3);
        self.edges.push(edge4);
        self.edges.push(edge5);
        self.edges.push(edge6);
    }

    fn construct_hull(&mut self) {
        let mut planes_to_add: Vec<Plane> = Vec::new();
        let mut planes_to_remove: Vec<Plane> = Vec::new();
        for plane in self.planes.iter() {
            let mut os: Vec<Point> = Vec::new();

            for point in self.points.iter() {
                if point_above_plane(&plane, &point) {
                    os.push(point.clone());
                }
            }

            if os.is_empty() {
                // planes_to_add.push(plane.clone());
                continue;
            } else {
                let farthest_point = match farthest_point_from_plane(&plane, &os) {
                    Some(p) => p,
                    None => continue,
                };
                println!("Adding point: {:?}", farthest_point);
                let plane1 = Plane::new(plane.point_a.clone(), plane.point_b.clone(), farthest_point.clone());
                let plane2 = Plane::new(plane.point_b.clone(), plane.point_c.clone(), farthest_point.clone());
                let plane3 = Plane::new(plane.point_c.clone(), plane.point_a.clone(), farthest_point.clone());
                planes_to_add.push(plane1);
                planes_to_add.push(plane2);
                planes_to_add.push(plane3);
                planes_to_remove.push(plane.clone());
            }
        }

        if planes_to_add.is_empty() {
            return;
        } else {
            println!("Planes to add: {}", planes_to_add.len());
        }

        for plane in planes_to_add.iter() {
            // println!("Plane: ({:?}, {:?}, {:?})", plane.point_a, plane.point_b, plane.point_c);
            // self.add_plane(plane.clone());
            if !self.planes.contains(plane) {
                self.add_plane(plane.clone());
            }
        }

        for plane in planes_to_remove.iter() {
            self.remove_plane(plane.clone());
        }

        self.construct_hull();
    }

    pub fn quick_hull(&mut self) {
        self.init_simplex();

        self.construct_hull();

        // remove duplicate planes
        let mut unique_planes: Vec<Plane> = Vec::new();
        for plane in self.planes.iter() {
            if !unique_planes.contains(plane) {
                unique_planes.push(plane.clone());
            }
        }
        self.planes = unique_planes;
        
    }

    pub fn add_plane(&mut self, plane: Plane) {
        let mut plane = plane;
        plane.calculate_normal();
        self.planes.push(plane);
    }

    pub fn remove_plane(&mut self, plane: Plane) {
        self.planes.remove(self.planes.iter().position(|p| *p == plane).unwrap());
    }

}
