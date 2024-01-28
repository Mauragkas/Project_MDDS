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
        Point { x: 2, y: 2, z: 2 },
        Point { x: 2, y: 0, z: 2 },
        Point { x: 0, y: 2, z: 2 },
        Point { x: 2, y: 2, z: 0 },
    ];

    let mut convex_hull = ConvexHull::new(points.to_vec());
    // let mut convex_hull = ConvexHull::new(create_rng_ponts(4));
    // convex_hull.initialize();
    convex_hull.gift_wrapping();

    // println!("{:?}", convex_hull);
    for i in 0..convex_hull.planes.len() {
        println!("Plane {}", i);
        println!("Point A: {:?}", convex_hull.planes[i].point_a);
        println!("Point B: {:?}", convex_hull.planes[i].point_b);
        println!("Point C: {:?}", convex_hull.planes[i].point_c);
        println!("Normal: {:?}", convex_hull.planes[i].normal);
        println!("Edge A: {:?}", convex_hull.planes[i].edge_a);
        println!("Edge B: {:?}", convex_hull.planes[i].edge_b);
        println!("Edge C: {:?}", convex_hull.planes[i].edge_c);
        println!();
    }

}
