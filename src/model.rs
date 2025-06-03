use crate::math::Float3;
use std::fs::read_to_string;

pub struct Model {
    pub triangle_points: Vec<Float3>,
    pub triangle_colors: Vec<Float3>,
}

impl Model {
    pub fn new(triangle_points: Vec<Float3>, triangle_colors: Vec<Float3>) -> Self {
        Self {
            triangle_points,
            triangle_colors,
        }
    }
}

pub fn read_obj_file(path: &str) -> std::io::Result<Vec<Float3>> {
    let mut points: Vec<Float3> = Vec::new();
    let mut triangle_points: Vec<Float3> = Vec::new();

    for line in read_to_string(path)?.lines() {
        if line.starts_with("v ") {
            let pts: Vec<f32> = line[2..]
                .split(" ")
                .map(|v| v.parse::<f32>().unwrap())
                .collect();

            points.push(Float3::new(pts[0], pts[1], pts[2]));
        } else if line.starts_with("f ") {
            let face_index_groups = line[2..].split(" ");

            for (i, index_group) in face_index_groups.enumerate() {
                // Face indices are always positive integers starting from 1
                let point_index = index_group
                    .split("/")
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()[0]
                    - 1;

                // Create triangle fan if there are more than three points in a face
                // First triangle corresponds to the first three points. Every next
                // triangle consists of first point in the face, the latest second to
                // last point, and a new point.
                // Assumes that faces are convex.
                if i >= 3 {
                    triangle_points.push(triangle_points[triangle_points.len() - (3 * i - 6)]);
                    triangle_points.push(triangle_points[triangle_points.len() - 2]);
                }
                triangle_points.push(points[point_index]);
            }
        }
    }

    Ok(triangle_points)
}
