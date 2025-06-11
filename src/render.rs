use core::f32;

use crate::camera::Camera;
use crate::math::{Float2, Float3, Float4, point_in_triangle};
use crate::model::Model;
use crate::scene::Scene;
use crate::transform::Transform;

#[derive(Debug, Clone, Copy)]
struct RasterizerPoint {
    screen_pos: Float2,
    texture_coords: Float2,
    normal: Float3,
    depth: f32,
}

impl RasterizerPoint {
    fn new(screen_pos: Float2, texture_coords: Float2, normal: Float3, depth: f32) -> Self {
        RasterizerPoint {
            screen_pos,
            texture_coords,
            normal,
            depth,
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
            size: Float2 {
                x: width as f32,
                y: height as f32,
            },
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
            let rasterizer_points =
                subdivide_partial_oob_triangles(model, &scene.camera, self.size);

            for chunk in rasterizer_points.chunks_exact(3) {
                let (
                    RasterizerPoint {
                        screen_pos: a,
                        texture_coords: uv_a,
                        normal: n_a,
                        depth: a_z,
                    },
                    RasterizerPoint {
                        screen_pos: b,
                        texture_coords: uv_b,
                        normal: n_b,
                        depth: b_z,
                    },
                    RasterizerPoint {
                        screen_pos: c,
                        texture_coords: uv_c,
                        normal: n_c,
                        depth: c_z,
                    },
                ) = (chunk[0], chunk[1], chunk[2]);

                // let depths = (1.0 + Float3::new(a_z, b_z, c_z)) * 0.5;
                let inverse_depths = 1.0 / Float3::new(a_z, b_z, c_z);

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

                            let uv = (uv_a * inverse_depths.x * weights.x
                                + uv_b * inverse_depths.y * weights.y
                                + uv_c * inverse_depths.z * weights.z)
                                * depth;
                            let normal = (n_a * inverse_depths.x * weights.x
                                + n_b * inverse_depths.y * weights.y
                                + n_c * inverse_depths.z * weights.z)
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
}

fn model_to_world(vertex: Float4, transform: &Transform) -> Float4 {
    transform.to_world_point(vertex)
}

fn world_to_view(vertex: Float4, camera: &Camera) -> Float4 {
    camera.transform.to_local_point(vertex)
}

fn view_to_screen(vertex: Float4, camera: &Camera, size: Float2) -> (Float2, f32) {
    // Perspective projection
    let mut vertex_persp = &camera.projection * vertex;
    // Make vector homogeneous again
    vertex_persp /= vertex_persp.w;

    // Non-invertible projection onto screen space
    let vertex_screen = Float2::new(
        (vertex_persp.x + 1.0) * 0.5 * size.x,
        (1.0 - (vertex_persp.y + 1.0) * 0.5) * size.y,
    );

    (vertex_screen, vertex.z)
}

fn subdivide_partial_oob_triangles(
    model: &Model,
    camera: &Camera,
    size: Float2,
) -> Vec<RasterizerPoint> {
    let mut rasterizer_points: Vec<RasterizerPoint> = Vec::new();
    for ((vertices, texture_coords), normals) in model
        .triangle_points
        .chunks_exact(3)
        .zip(model.texture_coords.chunks_exact(3))
        .zip(model.normals.chunks_exact(3))
    {
        let vertices_view = [
            world_to_view(
                model_to_world(Float4::from_point(vertices[0]), &model.transform),
                &camera,
            ),
            world_to_view(
                model_to_world(Float4::from_point(vertices[1]), &model.transform),
                &camera,
            ),
            world_to_view(
                model_to_world(Float4::from_point(vertices[2]), &model.transform),
                &camera,
            ),
        ];

        let clip_0 = vertices_view[0].z >= camera.near;
        let clip_1 = vertices_view[1].z >= camera.near;
        let clip_2 = vertices_view[2].z >= camera.near;
        let clip_count = clip_0 as usize + clip_1 as usize + clip_2 as usize;

        let m_rot = camera.transform.get_rotation() * model.transform.get_inverse_rotation();
        let normals = [
            &m_rot * Float4::from_vector(normals[0]),
            &m_rot * Float4::from_vector(normals[1]),
            &m_rot * Float4::from_vector(normals[2]),
        ];

        match clip_count {
            0 => {
                let ((a_screen, a_z), (b_screen, b_z), (c_screen, c_z)) = (
                    view_to_screen(vertices_view[0], &camera, size),
                    view_to_screen(vertices_view[1], &camera, size),
                    view_to_screen(vertices_view[2], &camera, size),
                );

                // let normals = [
                //     world_to_view(
                //         model_to_world(Float4::from_vector(normals[0]), &model.transform),
                //         &camera,
                //     ),
                //     world_to_view(
                //         model_to_world(Float4::from_vector(normals[1]), &model.transform),
                //         &camera,
                //     ),
                //     world_to_view(
                //         model_to_world(Float4::from_vector(normals[2]), &model.transform),
                //         &camera,
                //     ),
                // ];

                rasterizer_points.push(RasterizerPoint::new(
                    a_screen,
                    texture_coords[0],
                    normals[0].xyz(),
                    a_z,
                ));
                rasterizer_points.push(RasterizerPoint::new(
                    b_screen,
                    texture_coords[1],
                    normals[1].xyz(),
                    b_z,
                ));
                rasterizer_points.push(RasterizerPoint::new(
                    c_screen,
                    texture_coords[2],
                    normals[2].xyz(),
                    c_z,
                ));
            }
            1 => {
                // Determine which vertex will be clipped and which two will remain.
                let idx_clip = if clip_0 {
                    0
                } else {
                    if clip_1 { 1 } else { 2 }
                };
                let idx_next = (idx_clip + 1) % 3;
                let idx_prev = (idx_clip - 1 + 3) % 3;
                let vertex_clip = vertices_view[idx_clip];
                let vertex_a = vertices_view[idx_next];
                let vertex_b = vertices_view[idx_prev];

                // Fraction along triangle edge at which the depth is equal to the clip distance
                let frac_a = (camera.near - vertex_clip.z) / (vertex_a.z - vertex_clip.z);
                let frac_b = (camera.near - vertex_clip.z) / (vertex_b.z - vertex_clip.z);

                // New triangle points in view space
                let clip_vertex_along_edge_a = vertex_clip.lerp(vertex_a, frac_a);
                let clip_vertex_along_edge_b = vertex_clip.lerp(vertex_b, frac_b);

                let uv_a = texture_coords[idx_clip].lerp(texture_coords[idx_next], frac_a);
                let uv_b = texture_coords[idx_clip].lerp(texture_coords[idx_prev], frac_b);

                // let normals = [
                //     world_to_view(
                //         model_to_world(Float4::from_vector(normals[0]), &model.transform),
                //         &camera,
                //     ),
                //     world_to_view(
                //         model_to_world(Float4::from_vector(normals[1]), &model.transform),
                //         &camera,
                //     ),
                //     world_to_view(
                //         model_to_world(Float4::from_vector(normals[2]), &model.transform),
                //         &camera,
                //     ),
                // ];

                let normal_a = normals[idx_clip].lerp(normals[idx_next], frac_a);
                let normal_b = normals[idx_clip].lerp(normals[idx_prev], frac_b);

                // First new triangle
                let ((a_screen, a_z), (b_screen, b_z), (c_screen, c_z)) = (
                    view_to_screen(clip_vertex_along_edge_b, &camera, size),
                    view_to_screen(clip_vertex_along_edge_a, &camera, size),
                    view_to_screen(vertex_b, &camera, size),
                );
                rasterizer_points.push(RasterizerPoint::new(a_screen, uv_b, normal_b.xyz(), a_z));
                rasterizer_points.push(RasterizerPoint::new(b_screen, uv_a, normal_a.xyz(), b_z));
                rasterizer_points.push(RasterizerPoint::new(
                    c_screen,
                    texture_coords[idx_prev],
                    normals[idx_prev].xyz(),
                    c_z,
                ));

                // Second new triangle
                let ((a_screen, a_z), (b_screen, b_z), (c_screen, c_z)) = (
                    view_to_screen(clip_vertex_along_edge_a, &camera, size),
                    view_to_screen(vertex_a, &camera, size),
                    view_to_screen(vertex_b, &camera, size),
                );
                rasterizer_points.push(RasterizerPoint::new(a_screen, uv_a, normal_a.xyz(), a_z));
                rasterizer_points.push(RasterizerPoint::new(
                    b_screen,
                    texture_coords[idx_next],
                    normals[idx_next].xyz(),
                    b_z,
                ));
                rasterizer_points.push(RasterizerPoint::new(
                    c_screen,
                    texture_coords[idx_prev],
                    normals[idx_prev].xyz(),
                    c_z,
                ));
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
                let vertex_non_clip = vertices_view[idx_non_clip];
                let vertex_a = vertices_view[idx_next];
                let vertex_b = vertices_view[idx_prev];

                // Fraction along triangle edge at which the depth is equal to the clip distance
                let frac_a = (camera.near - vertex_non_clip.z) / (vertex_a.z - vertex_non_clip.z);
                let frac_b = (camera.near - vertex_non_clip.z) / (vertex_b.z - vertex_non_clip.z);

                // New triangle points in view space
                let clip_vertex_along_edge_a = vertex_non_clip.lerp(vertex_a, frac_a);
                let clip_vertex_along_edge_b = vertex_non_clip.lerp(vertex_b, frac_b);

                let uv_a = texture_coords[idx_non_clip].lerp(texture_coords[idx_next], frac_a);
                let uv_b = texture_coords[idx_non_clip].lerp(texture_coords[idx_prev], frac_b);

                // let normals = [
                //     world_to_view(
                //         model_to_world(Float4::from_vector(normals[0]), &model.transform),
                //         &camera,
                //     ),
                //     world_to_view(
                //         model_to_world(Float4::from_vector(normals[1]), &model.transform),
                //         &camera,
                //     ),
                //     world_to_view(
                //         model_to_world(Float4::from_vector(normals[2]), &model.transform),
                //         &camera,
                //     ),
                // ];

                let normal_a = normals[idx_non_clip].lerp(normals[idx_next], frac_a);
                let normal_b = normals[idx_non_clip].lerp(normals[idx_prev], frac_b);

                // New triangle
                let ((a_screen, a_z), (b_screen, b_z), (c_screen, c_z)) = (
                    view_to_screen(clip_vertex_along_edge_b, &camera, size),
                    view_to_screen(vertex_non_clip, &camera, size),
                    view_to_screen(clip_vertex_along_edge_a, &camera, size),
                );
                rasterizer_points.push(RasterizerPoint::new(a_screen, uv_b, normal_b.xyz(), a_z));
                rasterizer_points.push(RasterizerPoint::new(
                    b_screen,
                    texture_coords[idx_non_clip],
                    normals[idx_non_clip].xyz(),
                    b_z,
                ));
                rasterizer_points.push(RasterizerPoint::new(c_screen, uv_a, normal_a.xyz(), c_z));
            }
            _ => continue,
        }
    }

    rasterizer_points
}
