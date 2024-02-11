// #![allow(unused)]

mod convex_hull;
mod functions;
mod point;
mod plane;
mod edge;
mod hash_stuff;

use convex_hull::*;
use functions::*;
use point::*;

fn main() {
    // let mut points: Vec<Point> = vec![
    //     Point::new(None, 0, 0, 0),
    //     Point::new(None, 2, 0, 0),
    //     Point::new(None, 0, 2, 0),
    //     // Point::new(None, 0, 0, 2),
    //     // Point::new(None, 2, 2, 2),
    //     // Point::new(None, 3, 3, 3)
    // ];

    // points.extend(populate_point_vec());

    let points: Vec<Point> = populate_point_vec();

    // let points: Vec<Point> = create_rng_ponts(30);

    // points.extend(create_rng_ponts(3));

    let mut convex_hull = ConvexHull::new(points.to_vec());
    // let mut convex_hull = ConvexHull::new(create_rng_ponts(4));
    // convex_hull.initialize();
    // convex_hull.gift_wrapping();
    convex_hull.quick_hull();

    // println!("{:?}", convex_hull);
    // for plane in convex_hull.planes.iter() {
    //     println!("Plane: ({:?}, {:?}, {:?})", plane.point_a, plane.point_b, plane.point_c);
    // }

    println!("# of planes: {}", convex_hull.planes.len());

    // save the convex hull to a file
    save_to_json("convex_hull.json", &convex_hull);

    // println!("{:?}", get_ENV());
}
