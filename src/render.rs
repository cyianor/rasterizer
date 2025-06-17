use core::f32;
use std::ops::{Add, Mul};

use crate::camera::Camera;
use crate::math::{Float2, Float3, Float4, point_in_triangle, signed_triangle_area};
use crate::scene::Scene;
use crate::shader::ModelShader;

#[derive(Debug, Clone, Copy)]
pub struct VertexAttributes {
    pub vertex: Float3,
    pub uv: Float2,
    pub normal: Float3,
}

impl VertexAttributes {
    pub fn new(vertex: Float3, uv: Float2, normal: Float3) -> Self {
        Self { vertex, uv, normal }
    }

    pub fn lerp(&self, other: &VertexAttributes, proportion: f32) -> Self {
        let vertex = self.vertex.lerp(other.vertex, proportion);
        let uv = self.uv.lerp(other.uv, proportion);
        let normal = self.normal.lerp(other.normal, proportion);

        Self::new(vertex, uv, normal)
    }
}

impl Add for VertexAttributes {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            vertex: self.vertex + rhs.vertex,
            uv: self.uv + rhs.uv,
            normal: self.normal + rhs.normal,
        }
    }
}

impl Mul<f32> for VertexAttributes {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            vertex: self.vertex * rhs,
            uv: self.uv * rhs,
            normal: self.normal * rhs,
        }
    }
}

struct Triangle {
    vertices: [Float4; 3],
    vertex_attributes: [VertexAttributes; 3],
}

impl Triangle {
    pub fn new(vertices: [Float4; 3], vertex_attributes: [VertexAttributes; 3]) -> Self {
        Self {
            vertices,
            vertex_attributes,
        }
    }

    pub fn perspective_interpolation(
        &self,
        inverse_ws: Float3,
        w: f32,
        weights: Float3,
    ) -> VertexAttributes {
        (self.vertex_attributes[0] * (inverse_ws.x * weights.x)
            + self.vertex_attributes[1] * (inverse_ws.y * weights.y)
            + self.vertex_attributes[2] * (inverse_ws.z * weights.z))
            * w
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
        self.depth_buffer.fill(f32::INFINITY);
    }

    pub fn render(&mut self, scene: &Scene) {
        for model in scene.models.iter() {
            let model_world_matrix = model.transform.world_matrix();
            let camera_view_proj_matrix =
                &scene.camera.projection * scene.camera.transform.inverse_world_matrix();

            // Vertex shader
            let model_shader = ModelShader::new(model_world_matrix, camera_view_proj_matrix);
            let (vertices, vertices_attr, normals) =
                model_shader.transform(&model.vertices, &model.normals);

            // Assemble, cull, and subdivide (if necessary) triangles
            let triangles = model
                .vertex_indices
                .chunks_exact(3)
                .zip(model.texture_coord_indices.chunks_exact(3))
                .zip(model.normal_indices.chunks_exact(3))
                .filter(|((vs, _), _)| {
                    (vertices[vs[0]].1 & vertices[vs[1]].1 & vertices[vs[2]].1) == 0
                })
                .map(|((vs, uvs), ns)| {
                    Triangle::new(
                        [vertices[vs[0]].0, vertices[vs[1]].0, vertices[vs[2]].0],
                        [
                            VertexAttributes::new(
                                vertices_attr[vs[0]],
                                model.texture_coords[uvs[0]],
                                normals[ns[0]],
                            ),
                            VertexAttributes::new(
                                vertices_attr[vs[1]],
                                model.texture_coords[uvs[1]],
                                normals[ns[1]],
                            ),
                            VertexAttributes::new(
                                vertices_attr[vs[2]],
                                model.texture_coords[uvs[2]],
                                normals[ns[2]],
                            ),
                        ],
                    )
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

                let inverse_view_depths = 1.0
                    / Float3::new(
                        triangle.vertices[0].w,
                        triangle.vertices[1].w,
                        triangle.vertices[2].w,
                    );
                let depths =
                    (1.0 + Float3::new(
                        triangle.vertices[0].z / triangle.vertices[0].w,
                        triangle.vertices[1].z / triangle.vertices[1].w,
                        triangle.vertices[2].z / triangle.vertices[2].w,
                    )) * 0.5;

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
                            // Depth like in OpenGL
                            // Perspective projection leads to
                            // z' = ((far + near) / (far - near) - 2 * far * near / (z * (far - near)) + 1) / 2
                            // which is equivalent to (1/z - 1/near) / (1/far - 1/near) because
                            // (I) -2/z / ((far - near) / (far * near)) = -2/z / (1/near - 1/far) = 2/z / (1/far - 1/near)
                            // (II) (far + near) / (far - near) = (1/near + 1/far) / (1/near - 1/far)
                            // (III) ((2/z - (1/near + 1/far)) / (1/far - 1/near) + 1)/ 2
                            //     = (2/z - (1/near + 1/far) + (1/far - 1/near)) / (2 * (1/far - 1/near))
                            //     = (1/z - 1/near) / (1/far - 1/near) = a * 1/z + b
                            let depth = depths.dot(weights);
                            if depth > self.depth_buffer[y * self.width + x] || depth > 1.0 {
                                continue;
                            }

                            let attrs = triangle.perspective_interpolation(
                                inverse_view_depths,
                                1.0 / inverse_view_depths.dot(weights),
                                weights,
                            );

                            self.color_buffer[y * self.width + x] = model.shader.color(attrs);
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

    pub fn depth_buffer_to_byte_array(&self, bytes: &mut Vec<u8>, camera: &Camera) {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.depth_buffer[y * self.width + x];

                bytes[y * self.width + x] = if c == f32::INFINITY {
                    255
                } else {
                    // Reverse z coordinate projection
                    // z' = ((far + near) / (far - near) - 2 * far * near / (z * (far - near)) + 1) / 2
                    // (2*z' - 1) * (far - near) = (far + near) - 2 * far * near / z
                    // 2*far*near / z = far + near - (2*z' - 1) * (far - near)
                    // z = 2 * far * near / (far + near - (2*z' - 1) * (far - near))
                    //
                    // z is in [far, near] (note that far and near are negative)
                    // Divide by far to squish values into [0, 1]
                    // z values close to near will be almost 0 and values at far will be 1.
                    ((2.0 * camera.far * camera.near
                        / (camera.far + camera.near
                            - (2.0 * c - 1.0) * (camera.far - camera.near))
                        / camera.far)
                        * 255.0)
                        .clamp(0.0, 255.0) as u8
                };
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
            let clip_vertex_along_edge_a = vertex_clip.lerp(&vertex_a, frac_a);
            let clip_vertex_along_edge_b = vertex_clip.lerp(&vertex_b, frac_b);

            let attrs_a = triangle.vertex_attributes[idx_clip]
                .lerp(&triangle.vertex_attributes[idx_next], frac_a);
            let attrs_b = triangle.vertex_attributes[idx_clip]
                .lerp(&triangle.vertex_attributes[idx_prev], frac_b);

            // First new triangle
            Vec::from([
                Triangle::new(
                    [clip_vertex_along_edge_b, clip_vertex_along_edge_a, vertex_b],
                    [attrs_b, attrs_a, triangle.vertex_attributes[idx_prev]],
                ),
                Triangle::new(
                    [clip_vertex_along_edge_a, vertex_a, vertex_b],
                    [
                        attrs_a,
                        triangle.vertex_attributes[idx_next],
                        triangle.vertex_attributes[idx_prev],
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
            let clip_vertex_along_edge_a = vertex_non_clip.lerp(&vertex_a, frac_a);
            let clip_vertex_along_edge_b = vertex_non_clip.lerp(&vertex_b, frac_b);

            let attrs_a = triangle.vertex_attributes[idx_non_clip]
                .lerp(&triangle.vertex_attributes[idx_next], frac_a);
            let attrs_b = triangle.vertex_attributes[idx_non_clip]
                .lerp(&triangle.vertex_attributes[idx_prev], frac_b);

            // New triangle
            Vec::from([Triangle::new(
                [
                    clip_vertex_along_edge_b,
                    vertex_non_clip,
                    clip_vertex_along_edge_a,
                ],
                [attrs_b, triangle.vertex_attributes[idx_non_clip], attrs_a],
            )])
        }
        _ => Vec::new(),
    }
}
