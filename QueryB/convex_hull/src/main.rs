#![allow(unused)]
use rand::Rng;
use rand::distributions::Uniform;
use gnuplot::{AxesCommon, Caption, Color, Figure};
use serde::{Deserialize, Serialize};
use serde_json::json;

mod convex_hull;
mod functions;
mod point;
mod plane;
mod edge;
mod hash_stuff;

use convex_hull::*;
use functions::*;
use point::*;
use plane::*;
use edge::*;
use hash_stuff::*;

fn main() {
    // let mut points: Vec<Point> = vec![
    //     Point::new(None, 0, 0, 0),
    //     Point::new(None, 2, 0, 0),
    //     Point::new(None, 0, 2, 0),
    //     // Point { x: 0, y: 0, z: 2 },
    //     // Point { x: 2, y: 2, z: 22 },
    //     // Point { x: 3, y: 3, z: 3 },
    // ];

    // points.extend(create_rng_ponts(3));

    // let mut convex_hull = ConvexHull::new(points.to_vec());
    // // let mut convex_hull = ConvexHull::new(create_rng_ponts(4));
    // // convex_hull.initialize();
    // convex_hull.gift_wrapping();

    // // println!("{:?}", convex_hull);
    // // for plane in convex_hull.planes.iter() {
    // //     println!("Plane: ({:?}, {:?}, {:?})", plane.point_a, plane.point_b, plane.point_c);
    // // }

    // println!("# of planes: {}", convex_hull.planes.len());

    // // save the convex hull to a file
    // save_to_json("convex_hull.json", &convex_hull);

    // println!("{:?}", get_ENV());
    let points = populate_point_vec();

    for p in points.iter() {
        println!("({:?}, {:?}, {:?})", p.x, p.y, p.z);
    }
}
