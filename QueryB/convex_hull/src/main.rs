#![allow(unused)]
use rand::Rng;
use rand::distributions::Uniform;
use gnuplot::{AxesCommon, Caption, Color, Figure};

mod convex_hull;
mod functions;
mod point;
mod plane;
mod edge;

use convex_hull::*;
use functions::*;
use point::*;
use plane::*;

fn main() {
    let points = [
        Point { x: 0, y: 0, z: 0 },
        Point { x: 2, y: 0, z: 0 },
        Point { x: 0, y: 2, z: 0 },
        Point { x: 1, y: 1, z: 1 },
    ];

    let mut convex_hull = ConvexHull::new(points.to_vec());
    // let mut convex_hull = ConvexHull::new(create_rng_ponts(4));
    convex_hull.initialize();

    for edge in convex_hull.edges.iter() {
        print!("({}, {}, {}), ", edge.point1.x, edge.point1.y, edge.point1.z);
        println!("({}, {}, {}), ", edge.point2.x, edge.point2.y, edge.point2.z);
    }

    // // lets say we want to add another point
    // let point = Point { x: 2, y: 1, z: 1 };
    // let planes_to_check: Vec<_> = convex_hull.planes.iter().cloned().collect();

    // let mut possible_planes: Vec<Plane> = Vec::new();

    // for plane in planes_to_check {
    //     if point_above_plane(plane.clone(), point.clone()) {
    //         // println!("Point is above plane {:?}", plane);
    //         // println!("Point is above plane");
    //         possible_planes.push(plane.clone());
    //         // convex_hull.add_point(point.clone(), plane.clone());
    //         // break;
    //     }
    // }


    // println!("{:?} is above planes: ", point);
    // for plane in possible_planes {
    //     print!("Planes: ");
    //     print!("({}, {}, {}), ", plane.point_a.x, plane.point_a.y, plane.point_a.z);
    //     print!("({}, {}, {}), ", plane.point_b.x, plane.point_b.y, plane.point_b.z);
    //     println!("({}, {}, {}), ", plane.point_c.x, plane.point_c.y, plane.point_c.z);

    // }

    // println!();

    // for plane in convex_hull.planes.iter() {
    //     print!("Plane[{}]: ", convex_hull.planes.iter().position(|p| p == plane).unwrap());
    //     for point in plane.get_points().iter() {
    //         print!("({}, {}, {}), ", point.x, point.y, point.z);
    //     }
    //     println!("Normal: ({}, {}, {})", plane.normal.x, plane.normal.y, plane.normal.z);
    // }

    // let mut fg = Figure::new();

    // // // Define a scatter plot of the points
    // // fg.axes3d()
    // //     .points(
    // //         convex_hull.points.iter().map(|p| p.x),
    // //         convex_hull.points.iter().map(|p| p.y),
    // //         convex_hull.points.iter().map(|p| p.z),
    // //         &[Caption("Points"), Color("red")],
    // //     );
    // fg.axes3d()
    //     .lines(
    //         convex_hull.edges.iter().map(|e| e.point1.x),
    //         convex_hull.edges.iter().map(|e| e.point1.y),
    //         convex_hull.edges.iter().map(|e| e.point1.z),
    //         &[Caption("Edges"), Color("blue")],
    //     );

    // // Show the plot in a window
    // fg.show().unwrap();
}
