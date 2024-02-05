#![allow(unused)]
use rand::Rng;
use rand::distributions::Uniform;

use std::io::Write;
use std::fs::File;
use std::path::Path;

use crate::convex_hull::*;
use crate::point::*;
use crate::plane::*;
use crate::hash_stuff::*;

const FILE_LOCATION: &str = "../../pol.json";

pub fn create_rng_ponts(iterations: u32) -> Vec<Point> {
    let vec: Vec<Point> = (0..iterations)
        .map(|_| Point::new(
            None, 
            rand::thread_rng().sample(Uniform::new(0, 20)),
            rand::thread_rng().sample(Uniform::new(0, 20)),
            rand::thread_rng().sample(Uniform::new(0, 20)),
        ))
        .collect();
    vec
}

pub fn populate_point_vec() -> Vec<Point> {
    // open the FILE_LOCATION json file and read the contents
    let file = std::fs::read_to_string(FILE_LOCATION).unwrap();
    let data: Vec<Data> = serde_json::from_str(&file).unwrap();
    let mut points: Vec<Point> = Vec::new();
    for d in data.iter() {
        points.push(Point::new(
            Some(d.clone()), 
            hash(&d.dblp_record, get_ENV().get_DBLP_RECORDS_LENGTH()) as i32, 
            hash(&d.surname, get_ENV().get_SURNAMES_LENGTH()) as i32, 
            d.year_of_release));
    }
    points
}

pub fn cross_product(a: Point, b: Point) -> Point {
    Point::new(
        None,
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

// pub fn dot_product(a: Point, b: Point) -> f64 {
pub fn dot_product(a: &Point, b: &Point) -> f64 {
    (a.x * b.x + a.y * b.y + a.z * b.z) as f64
}

pub fn subtract_vectors(a: Point, b: Point) -> Point {
    Point::new(
        None,
        a.x - b.x,
        a.y - b.y,
        a.z - b.z,
    )
}

pub fn point_above_plane(plane: Plane, point: Point) -> bool {
    let vector1 = subtract_vectors(plane.point_b, plane.point_a.clone());
    let vector2 = subtract_vectors(plane.point_c, plane.point_a.clone());
    let normal = cross_product(vector1, vector2);
    let point_vector = subtract_vectors(point, plane.point_a);
    dot_product(&normal, &point_vector) > 0.0
}

pub fn angle_between_vectors(normal: &Point, vector: &Point) -> f64 {
    let dot = dot_product(&normal, &vector);
    let normal_length = (normal.x.pow(2) + normal.y.pow(2) + normal.z.pow(2)) as f64;
    let vector_length = (vector.x.pow(2) + vector.y.pow(2) + vector.z.pow(2)) as f64;
    let magnitude = (normal_length * vector_length).sqrt();
    let angle = dot / magnitude;
    if normal != vector {
        // println!("Angle normal {:?} and vector {:?} is {}", normal, vector, angle.to_degrees());
    }
    angle.to_degrees()
}

pub fn save_to_json<T>(filename: &str, data: &T)
where
    T: serde::Serialize,
{
    let serialized = serde_json::to_string_pretty(data).unwrap();

    let mut file = std::fs::File::create(filename).unwrap();

    file.write_all(serialized.as_bytes()).unwrap();
}

