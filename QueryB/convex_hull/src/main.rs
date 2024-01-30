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
        Point { x: 0, y: 0, z: 2 },
        Point { x: 2, y: 2, z: 22 },
        Point { x: 3, y: 3, z: 3 },
    ];

    let mut convex_hull = ConvexHull::new(points.to_vec());
    // let mut convex_hull = ConvexHull::new(create_rng_ponts(4));
    // convex_hull.initialize();
    convex_hull.gift_wrapping();

    // println!("{:?}", convex_hull);
    // for plane in convex_hull.planes.iter() {
    //     println!("Plane: ({:?}, {:?}, {:?})", plane.point_a, plane.point_b, plane.point_c);
    // }

}
