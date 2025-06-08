use crate::math::{Float2, Float3};
use crate::transform::Transform;
use std::fs::read_to_string;

pub struct Model {
    pub triangle_points: Vec<Float3>,
    pub triangle_colors: Vec<Float3>,
    pub texture_coords: Vec<Float2>,
    pub normals: Vec<Float3>,
    pub transform: Transform,
}

impl Model {
    pub fn new(
        triangle_points: Vec<Float3>,
        triangle_colors: Vec<Float3>,
        texture_coords: Vec<Float2>,
        normals: Vec<Float3>,
        transform: Transform,
    ) -> Self {
        Self {
            triangle_points,
            triangle_colors,
            texture_coords,
            normals,
            transform,
        }
    }
}

pub fn read_obj_file(
    path: &str,
    parse_texture_coords: bool,
    parse_normals: bool,
) -> std::io::Result<(Vec<Float3>, Vec<Float2>, Vec<Float3>)> {
    let mut vs: Vec<Float3> = Vec::new();
    let mut vts: Vec<Float2> = Vec::new();
    let mut vns: Vec<Float3> = Vec::new();
    let mut vertices: Vec<Float3> = Vec::new();
    let mut texture_coords: Vec<Float2> = Vec::new();
    let mut normals: Vec<Float3> = Vec::new();

    for line in read_to_string(path)?.lines() {
        if line.starts_with("v ") {
            let v: Vec<f32> = line[2..]
                .split(" ")
                .map(|v| v.parse::<f32>().unwrap())
                .collect();

            vs.push(Float3::new(v[0], v[1], v[2]));
        } else if line.starts_with("vn ") {
            let vn: Vec<f32> = line[3..]
                .split(" ")
                .map(|v| v.parse::<f32>().unwrap())
                .collect();

            vns.push(Float3::new(vn[0], vn[1], vn[2]));
        } else if line.starts_with("vt ") {
            let vt: Vec<f32> = line[3..]
                .split(" ")
                .map(|v| v.parse::<f32>().unwrap())
                .collect();

            vts.push(Float2::new(vt[0], vt[1]));
        } else if line.starts_with("f ") {
            let face_index_groups = line[2..].split(" ");

            for (i, index_group) in face_index_groups.enumerate() {
                // Face indices are always positive integers starting from 1
                let indices = index_group
                    .split("/")
                    .map(|v| v.parse::<usize>())
                    .collect::<Vec<_>>();

                // Create triangle fan if there are more than three points in a face
                // First triangle corresponds to the first three points. Every next
                // triangle consists of first point in the face, the latest second to
                // last point, and a new point.
                // Assumes that faces are convex.
                if i >= 3 {
                    vertices.push(vertices[vertices.len() - (3 * i - 6)]);
                    vertices.push(vertices[vertices.len() - 2]);

                    if parse_normals {
                        normals.push(normals[normals.len() - (3 * i - 6)]);
                        normals.push(normals[normals.len() - 2]);
                    }
                }
                vertices.push(vs[indices[0].as_ref().unwrap() - 1]);

                if parse_texture_coords {
                    if i >= 3 {
                        texture_coords.push(texture_coords[texture_coords.len() - (3 * i - 6)]);
                        texture_coords.push(texture_coords[texture_coords.len() - 2]);
                    }
                    texture_coords.push(vts[indices[1].as_ref().unwrap() - 1]);
                }

                if parse_normals {
                    if i >= 3 {
                    normals.push(normals[normals.len() - (3 * i - 6)]);
                    normals.push(normals[normals.len() - 2]);
                    }
                    normals.push(vns[indices[2].as_ref().unwrap() - 1]);
                }
            }
        }
    }

    Ok((vertices, texture_coords, normals))
}
