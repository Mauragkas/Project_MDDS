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

        // create the first edges
        let edge1 = Edge {
            point1: points[0].clone(),
            point2: points[1].clone(),
        };

        let edge2 = Edge {
            point1: points[1].clone(),
            point2: points[2].clone(),
        };

        let edge3 = Edge {
            point1: points[2].clone(),
            point2: points[0].clone(),
        };

        // add the edges to the edges vector
        self.edges.push(edge1.clone());
        self.edges.push(edge2.clone());
        self.edges.push(edge3.clone());

        // create the first plane
        let plane = Plane {
            point_a: points[0].clone(),
            point_b: points[1].clone(),
            point_c: points[2].clone(),
            normal: Point { x: 0, y: 0, z: 0 },
            edge_a: edge1.clone(),
            edge_b: edge2.clone(),
            edge_c: edge3.clone(),
        };

        // add the plane to the planes vector
        self.add_plane(plane);

    }

    pub fn gift_wrapping(&mut self) {
        self.gift_wrapping_init();

        // println!("Edges: {:?}", self.edges);

        println!("Planes: ");
        for plane in self.planes.iter() {
            println!("\t({:?}, {:?}, {:?})", plane.point_a, plane.point_b, plane.point_c);
        }

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
                let plane = self.planes.iter().find(|p| p.edge_a == *edge || p.edge_b == *edge || p.edge_c == *edge).unwrap();
                // println!("Normal: {:?}", plane.normal);
                let mut possible_planes: Vec<Plane> = Vec::new();
                for point in self.points.iter() {
                    // create a plane from the edge and the point
                    if point != &edge.point1 && point != &edge.point2 {
                        let mut plane = Plane::new(edge.point1.clone(), edge.point2.clone(), point.clone());
                        // push the plane to the possible planes vector
                        possible_planes.push(plane);
                    }
                }

                if possible_planes.contains(&plane) {
                    possible_planes.remove(possible_planes.iter().position(|p| *p == plane.clone()).unwrap());
                }

                // find the possible plane that has the minimum angle between its normal and the normal of the plane that contains the edge
                let mut min_angle = 360.0;
                let mut min_angle_plane = Plane::new(Point { x: 0, y: 0, z: 0 }, Point { x: 0, y: 0, z: 0 }, Point { x: 0, y: 0, z: 0 });
                for plane in possible_planes.iter() {
                    let angle = angle_between_vectors(plane.normal.clone(), plane.normal.clone());
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
            println!("Planes now: ");
            for plane in self.planes.iter() {
                println!("\t({:?}, {:?}, {:?})", plane.point_a, plane.point_b, plane.point_c);
            }
        }
        
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
