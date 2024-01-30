#![allow(unused)]
use rand::Rng;
use rand::distributions::Uniform;

use crate::convex_hull::*;
use crate::point::*;
use crate::plane::*;

pub fn create_rng_ponts(iterations: u32) -> Vec<Point> {
    let vec: Vec<Point> = (0..iterations)
        .map(|_| Point {
            x: rand::thread_rng().sample(Uniform::new(0, 20)),
            y: rand::thread_rng().sample(Uniform::new(0, 20)),
            z: rand::thread_rng().sample(Uniform::new(0, 20)),
        })
        .collect();
    vec
}

pub fn cross_product(a: Point, b: Point) -> Point {
    Point {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

pub fn dot_product(a: Point, b: Point) -> f64 {
    (a.x * b.x + a.y * b.y + a.z * b.z) as f64
}

pub fn subtract_vectors(a: Point, b: Point) -> Point {
    Point {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
}

pub fn point_above_plane(plane: Plane, point: Point) -> bool {
    let vector1 = subtract_vectors(plane.point_b, plane.point_a.clone());
    let vector2 = subtract_vectors(plane.point_c, plane.point_a.clone());
    let normal = cross_product(vector1, vector2);
    let point_vector = subtract_vectors(point, plane.point_a);
    dot_product(normal, point_vector) > 0.0
    // dot_product(normal, point_vector) < 0
}

pub fn angle_between_vectors(normal: Point, vector: Point) -> f64 {
    let dot = dot_product(normal.clone(), vector.clone());
    let normal_length = (normal.x.pow(2) + normal.y.pow(2) + normal.z.pow(2)) as f64;
    let vector_length = (vector.x.pow(2) + vector.y.pow(2) + vector.z.pow(2)) as f64;
    let magnitude = (normal_length * vector_length).sqrt();
    let angle = dot / magnitude;
    if normal != vector {
        println!("Angle normal {:?} and vector {:?} is {}", normal, vector, angle.to_degrees());
    }
    angle.to_degrees()
}