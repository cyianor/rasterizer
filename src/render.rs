use core::f32;

use crate::math::{Float2, Float3, Float4, point_in_triangle, signed_triangle_area};
use crate::scene::Scene;
use crate::shader::ModelShader;

struct Triangle {
    vertices: [Float4; 3],
    uvs: [Float2; 3],
    normals: [Float3; 3],
}

impl Triangle {
    pub fn new(vertices: [Float4; 3], uvs: [Float2; 3], normals: [Float3; 3]) -> Self {
        Self {
            vertices,
            uvs,
            normals,
        }
    }
}

pub struct RenderTarget {
    pub width: usize,
    pub height: usize,
    pub size: Float2,
    pub color_buffer: Vec<Float3>,
    pub depth_buffer: Vec<f32>,
}

impl RenderTarget {
    pub fn new(width: usize, height: usize) -> Self {
        let mut color_buffer: Vec<Float3> = Vec::new();
        color_buffer.resize(width * height, Float3::zeros());
        let mut depth_buffer: Vec<f32> = Vec::new();
        depth_buffer.resize(width * height, f32::INFINITY);

        Self {
            width,
            height,
            size: Float2::new(width as f32, height as f32),
            color_buffer,
            depth_buffer,
        }
    }

    pub fn clear(&mut self, color: Float3) {
        self.color_buffer.fill(color);
        self.depth_buffer.fill(f32::NEG_INFINITY);
    }

    pub fn render(&mut self, scene: &Scene) {
        for model in scene.models.iter() {
            let world_rotation_matrix = model.transform.get_inverse_rotation();
            let transformation = &scene.camera.projection
                * scene.camera.transform.inverse_world_matrix()
                * model.transform.world_matrix();

            // Vertex shader
            let model_shader = ModelShader::new(transformation, world_rotation_matrix);
            let (vertices, normals) = model_shader.transform(&model.vertices, &model.normals);

            // Assemble and cull triangles
            let triangles = model
                .vertex_indices
                .chunks_exact(3)
                .zip(model.texture_coord_indices.chunks_exact(3))
                .zip(model.normal_indices.chunks_exact(3))
                .filter_map(|((vs, uvs), ns)| {
                    if (vertices[vs[0]].1 & vertices[vs[1]].1 & vertices[vs[2]].1) == 0 {
                        Some(Triangle::new(
                            [vertices[vs[0]].0, vertices[vs[1]].0, vertices[vs[2]].0],
                            [
                                model.texture_coords[uvs[0]],
                                model.texture_coords[uvs[1]],
                                model.texture_coords[uvs[2]],
                            ],
                            [normals[ns[0]], normals[ns[1]], normals[ns[2]]],
                        ))
                    } else {
                        None
                    }
                })
                .flat_map(|triangle| subdivide_partial_oob_triangles(triangle));

            for triangle in triangles {
                let [a, b, c] = [
                    homogeneous_to_screen(triangle.vertices[0], self.size),
                    homogeneous_to_screen(triangle.vertices[1], self.size),
                    homogeneous_to_screen(triangle.vertices[2], self.size),
                ];

                // Back-face culling
                if signed_triangle_area(a, b, c) < 0.0 {
                    continue;
                }

                let inverse_depths = 1.0
                    / Float3::new(
                        triangle.vertices[0].w,
                        triangle.vertices[1].w,
                        triangle.vertices[2].w,
                    );

                // Determine chunk bounding box
                let (min_x, min_y, max_x, max_y) = (
                    a.x.min(b.x).min(c.x),
                    a.y.min(b.y).min(c.y),
                    a.x.max(b.x).max(c.x),
                    a.y.max(b.y).max(c.y),
                );

                let (bbox_start_x, bbox_start_y, bbox_end_x, bbox_end_y) = (
                    min_x.floor().clamp(0.0, self.size.x) as usize,
                    min_y.floor().clamp(0.0, self.size.y) as usize,
                    max_x.ceil().clamp(0.0, self.size.x) as usize,
                    max_y.ceil().clamp(0.0, self.size.y) as usize,
                );

                for y in bbox_start_y..bbox_end_y {
                    for x in bbox_start_x..bbox_end_x {
                        if let Some(weights) =
                            point_in_triangle(a, b, c, Float2::new(x as f32, y as f32))
                        {
                            let depth = 1.0 / inverse_depths.dot(weights);
                            if depth < self.depth_buffer[y * self.width + x] {
                                continue;
                            }

                            let uv = (triangle.uvs[0] * inverse_depths.x * weights.x
                                + triangle.uvs[1] * inverse_depths.y * weights.y
                                + triangle.uvs[2] * inverse_depths.z * weights.z)
                                * depth;
                            let normal = (triangle.normals[0] * inverse_depths.x * weights.x
                                + triangle.normals[1] * inverse_depths.y * weights.y
                                + triangle.normals[2] * inverse_depths.z * weights.z)
                                * depth;

                            self.color_buffer[y * self.width + x] = model.shader.color(uv, normal);
                            self.depth_buffer[y * self.width + x] = depth;
                        }
                    }
                }
            }
        }
    }

    pub fn color_buffer_to_byte_array(&self, bytes: &mut Vec<u8>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.color_buffer[y * self.width + x] * 255.0;
                bytes[(y * self.width + x) * 4 + 1] = c.y.clamp(0.0, 255.0) as u8;
                bytes[(y * self.width + x) * 4 + 0] = c.x.clamp(0.0, 255.0) as u8;
                bytes[(y * self.width + x) * 4 + 2] = c.z.clamp(0.0, 255.0) as u8;
                bytes[(y * self.width + x) * 4 + 3] = 255;
            }
        }
    }

    pub fn depth_buffer_to_byte_array(&self, bytes: &mut Vec<u8>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = -1.0 / self.depth_buffer[y * self.width + x];
                bytes[(y * self.width + x) * 4 + 1] = c.clamp(0.0, 255.0) as u8;
                bytes[(y * self.width + x) * 4 + 0] = c.clamp(0.0, 255.0) as u8;
                bytes[(y * self.width + x) * 4 + 2] = c.clamp(0.0, 255.0) as u8;
                bytes[(y * self.width + x) * 4 + 3] = 255;
            }
        }
    }
}

fn homogeneous_to_screen(vertex: Float4, size: Float2) -> Float2 {
    Float2::new(
        (vertex.x / vertex.w + 1.0) * 0.5 * size.x,
        (1.0 - (vertex.y / vertex.w + 1.0) * 0.5) * size.y,
    )
}

fn subdivide_partial_oob_triangles(triangle: Triangle) -> Vec<Triangle> {
    let vertices = triangle.vertices;

    // Near plane clipping
    let clip_0 = vertices[0].w >= 0.0 || vertices[0].z + vertices[0].w >= 0.0;
    let clip_1 = vertices[1].w >= 0.0 || vertices[1].z + vertices[1].w >= 0.0;
    let clip_2 = vertices[2].w >= 0.0 || vertices[2].z + vertices[2].w >= 0.0;
    let clip_count = clip_0 as usize + clip_1 as usize + clip_2 as usize;

    match clip_count {
        0 => Vec::from([triangle]),
        1 => {
            // Determine which vertex will be clipped and which two will remain.
            let idx_clip = if clip_0 {
                0
            } else {
                if clip_1 { 1 } else { 2 }
            };
            let idx_next = (idx_clip + 1) % 3;
            let idx_prev = (idx_clip - 1 + 3) % 3;
            let vertex_clip = vertices[idx_clip];
            let vertex_a = vertices[idx_next];
            let vertex_b = vertices[idx_prev];

            // Fraction along triangle edge at which the depth is equal to the clip distance
            let frac_a = (vertex_clip.w + vertex_clip.z)
                / ((vertex_clip.w + vertex_clip.z) - (vertex_a.w + vertex_a.z));
            let frac_b = (vertex_clip.w + vertex_clip.z)
                / ((vertex_clip.w + vertex_clip.z) - (vertex_b.w + vertex_b.z));

            // New triangle points in view space
            let clip_vertex_along_edge_a = vertex_clip.lerp(vertex_a, frac_a);
            let clip_vertex_along_edge_b = vertex_clip.lerp(vertex_b, frac_b);

            let uv_a = triangle.uvs[idx_clip].lerp(triangle.uvs[idx_next], frac_a);
            let uv_b = triangle.uvs[idx_clip].lerp(triangle.uvs[idx_prev], frac_b);

            let normal_a = triangle.normals[idx_clip].lerp(triangle.normals[idx_next], frac_a);
            let normal_b = triangle.normals[idx_clip].lerp(triangle.normals[idx_prev], frac_b);

            // First new triangle
            Vec::from([
                Triangle::new(
                    [clip_vertex_along_edge_b, clip_vertex_along_edge_a, vertex_b],
                    [uv_b, uv_a, triangle.uvs[idx_prev]],
                    [normal_b, normal_a, triangle.normals[idx_prev]],
                ),
                Triangle::new(
                    [clip_vertex_along_edge_a, vertex_a, vertex_b],
                    [uv_a, triangle.uvs[idx_next], triangle.uvs[idx_prev]],
                    [
                        normal_a,
                        triangle.normals[idx_next],
                        triangle.normals[idx_prev],
                    ],
                ),
            ])
        }
        2 => {
            // Figure out which point remains and the two that will be clipped
            let idx_non_clip = if !clip_0 {
                0
            } else {
                if !clip_1 { 1 } else { 2 }
            };
            let idx_next = (idx_non_clip + 1) % 3;
            let idx_prev = (idx_non_clip - 1 + 3) % 3;
            let vertex_non_clip = vertices[idx_non_clip];
            let vertex_a = vertices[idx_next];
            let vertex_b = vertices[idx_prev];

            // Fraction along triangle edge at which the depth is equal to the clip distance
            let frac_a = (vertex_non_clip.w + vertex_non_clip.z)
                / ((vertex_non_clip.w + vertex_non_clip.z) - (vertex_a.w + vertex_a.z));
            let frac_b = (vertex_non_clip.w + vertex_non_clip.z)
                / ((vertex_non_clip.w + vertex_non_clip.z) - (vertex_b.w + vertex_b.z));

            // New triangle points in view space
            let clip_vertex_along_edge_a = vertex_non_clip.lerp(vertex_a, frac_a);
            let clip_vertex_along_edge_b = vertex_non_clip.lerp(vertex_b, frac_b);

            let uv_a = triangle.uvs[idx_non_clip].lerp(triangle.uvs[idx_next], frac_a);
            let uv_b = triangle.uvs[idx_non_clip].lerp(triangle.uvs[idx_prev], frac_b);

            let normal_a = triangle.normals[idx_non_clip].lerp(triangle.normals[idx_next], frac_a);
            let normal_b = triangle.normals[idx_non_clip].lerp(triangle.normals[idx_prev], frac_b);

            // New triangle
            Vec::from([Triangle::new(
                [
                    clip_vertex_along_edge_b,
                    vertex_non_clip,
                    clip_vertex_along_edge_a,
                ],
                [uv_b, triangle.uvs[idx_non_clip], uv_a],
                [normal_b, triangle.normals[idx_non_clip], normal_a],
            )])
        }
        _ => Vec::new(),
    }
}
