use crate::math::{Float2, Float3};
use crate::shader::PixelShader;
use crate::transform::Transform;
use std::fs::read_to_string;

/// A model described by triangles with normals and texture coordinates
pub struct Model {
    /// Vertices making up the model
    pub vertices: Vec<Float3>,
    /// Indices in groups of 3 describing the triangles.
    /// Indices are with referece to [vertices](Model::vertices)
    pub vertex_indices: Vec<usize>,
    /// Texture coordinates
    pub texture_coords: Vec<Float2>,
    /// Indices of texture coordinates in groups of 3 for each triangle.
    /// Indices are with referece to [texture_coords](Model::texture_coords)
    pub texture_coord_indices: Vec<usize>,
    /// Normals at each vertex
    pub normals: Vec<Float3>,
    /// Indices of normals in groups of 3 for each triangle.
    /// Indices are with referece to [normals](Model::normals)
    pub normal_indices: Vec<usize>,
    /// Transformation of the model
    pub transform: Transform,
    /// Pixel shader applied to model
    pub shader: Box<dyn PixelShader>,
}

impl Model {
    /// Create a new model
    pub fn new(
        vertices: Vec<Float3>,
        vertex_indices: Vec<usize>,
        texture_coords: Vec<Float2>,
        texture_coord_indices: Vec<usize>,
        normals: Vec<Float3>,
        normal_indices: Vec<usize>,
        transform: Transform,
        shader: Box<dyn PixelShader>,
    ) -> Self {
        Self {
            vertices,
            vertex_indices,
            texture_coords,
            texture_coord_indices,
            normals,
            normal_indices,
            transform,
            shader,
        }
    }
}

/// Read an OBJ file
/// 
/// Upon success, the function returns a tuple containing
/// vertices, vertex_indices, texture_coords, texture_coord_indices, normals, and normal_indices.
pub fn read_obj_file(
    path: &str,
) -> std::io::Result<(
    Vec<Float3>,
    Vec<usize>,
    Vec<Float2>,
    Vec<usize>,
    Vec<Float3>,
    Vec<usize>,
)> {
    let mut vertices: Vec<Float3> = Vec::new();
    let mut texture_coords: Vec<Float2> = Vec::new();
    let mut normals: Vec<Float3> = Vec::new();
    let mut vertex_indices: Vec<usize> = Vec::new();
    let mut texture_coord_indices: Vec<usize> = Vec::new();
    let mut normal_indices: Vec<usize> = Vec::new();

    for line in read_to_string(path)?.lines() {
        if line.starts_with("v ") {
            let v: Vec<f32> = line[2..]
                .split(" ")
                .map(|v| v.parse::<f32>().unwrap())
                .collect();

            vertices.push(Float3::new(v[0], v[1], v[2]));
        } else if line.starts_with("vn ") {
            let vn: Vec<f32> = line[3..]
                .split(" ")
                .map(|v| v.parse::<f32>().unwrap())
                .collect();

            normals.push(Float3::new(vn[0], vn[1], vn[2]));
        } else if line.starts_with("vt ") {
            let vt: Vec<f32> = line[3..]
                .split(" ")
                .map(|v| v.parse::<f32>().unwrap())
                .collect();

            texture_coords.push(Float2::new(vt[0], vt[1]));
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
                    vertex_indices.push(vertex_indices[vertex_indices.len() - (3 * i - 6)]);
                    vertex_indices.push(vertex_indices[vertex_indices.len() - 2]);
                }
                vertex_indices.push(indices[0].as_ref().unwrap() - 1);

                if i >= 3 {
                    texture_coord_indices
                        .push(texture_coord_indices[texture_coord_indices.len() - (3 * i - 6)]);
                    texture_coord_indices
                        .push(texture_coord_indices[texture_coord_indices.len() - 2]);
                }
                texture_coord_indices.push(indices[1].as_ref().unwrap() - 1);

                if i >= 3 {
                    normal_indices.push(normal_indices[normal_indices.len() - (3 * i - 6)]);
                    normal_indices.push(normal_indices[normal_indices.len() - 2]);
                }
                normal_indices.push(indices[2].as_ref().unwrap() - 1);
            }
        }
    }

    Ok((
        vertices,
        vertex_indices,
        texture_coords,
        texture_coord_indices,
        normals,
        normal_indices,
    ))
}
