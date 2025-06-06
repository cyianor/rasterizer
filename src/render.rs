use core::f32;

use crate::camera::Camera;
use crate::math::{Float2, Float3, point_in_triangle};
use crate::scene::Scene;
use crate::transform::Transform;

#[derive(Clone, Copy)]
struct RasterizerPoint {
    screen_pos: Float2,
    depth: f32,
}

impl RasterizerPoint {
    fn new(screen_pos: Float2, depth: f32) -> Self {
        RasterizerPoint { screen_pos, depth }
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
        self.depth_buffer.fill(f32::INFINITY);
    }

    pub fn render(&mut self, scene: &Scene) {
        for model in scene.models.iter() {
            let mut rasterizer_points: Vec<RasterizerPoint> = Vec::new();
            let mut rasterize_colors: Vec<Float3> = Vec::new();
            for (chunk, color) in model
                .triangle_points
                .chunks_exact(3)
                .zip(model.triangle_colors.iter())
            {
                let vertices_view = [
                    world_to_view(model_to_world(chunk[0], &model.transform), &scene.camera),
                    world_to_view(model_to_world(chunk[1], &model.transform), &scene.camera),
                    world_to_view(model_to_world(chunk[2], &model.transform), &scene.camera),
                ];

                let clip_0 = vertices_view[0].z >= scene.camera.near;
                let clip_1 = vertices_view[1].z >= scene.camera.near;
                let clip_2 = vertices_view[2].z >= scene.camera.near;
                let clip_count = clip_0 as usize + clip_1 as usize + clip_2 as usize;

                match clip_count {
                    0 => {
                        let ((a_screen, a_z), (b_screen, b_z), (c_screen, c_z)) = (
                            view_to_screen(vertices_view[0], &scene.camera, self.size),
                            view_to_screen(vertices_view[1], &scene.camera, self.size),
                            view_to_screen(vertices_view[2], &scene.camera, self.size),
                        );
                        rasterizer_points.push(RasterizerPoint::new(a_screen, a_z));
                        rasterizer_points.push(RasterizerPoint::new(b_screen, b_z));
                        rasterizer_points.push(RasterizerPoint::new(c_screen, c_z));
                        rasterize_colors.push(*color);
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
                        let frac_a =
                            (scene.camera.near - vertex_clip.z) / (vertex_a.z - vertex_clip.z);
                        let frac_b =
                            (scene.camera.near - vertex_clip.z) / (vertex_b.z - vertex_clip.z);

                        // New triangle points in view space
                        let clip_vertex_along_edge_a = vertex_clip.lerp(vertex_a, frac_a);
                        let clip_vertex_along_edge_b = vertex_clip.lerp(vertex_b, frac_b);

                        // First new triangle
                        let ((a_screen, a_z), (b_screen, b_z), (c_screen, c_z)) = (
                            view_to_screen(clip_vertex_along_edge_b, &scene.camera, self.size),
                            view_to_screen(clip_vertex_along_edge_a, &scene.camera, self.size),
                            view_to_screen(vertex_b, &scene.camera, self.size),
                        );
                        rasterizer_points.push(RasterizerPoint::new(a_screen, a_z));
                        rasterizer_points.push(RasterizerPoint::new(b_screen, b_z));
                        rasterizer_points.push(RasterizerPoint::new(c_screen, c_z));
                        // rasterize_colors.push(*color);
                        rasterize_colors.push(Float3::new(1.0, 0.0, 0.0));

                        // Second new triangle
                        let ((a_screen, a_z), (b_screen, b_z), (c_screen, c_z)) = (
                            view_to_screen(clip_vertex_along_edge_a, &scene.camera, self.size),
                            view_to_screen(vertex_a, &scene.camera, self.size),
                            view_to_screen(vertex_b, &scene.camera, self.size),
                        );
                        rasterizer_points.push(RasterizerPoint::new(a_screen, a_z));
                        rasterizer_points.push(RasterizerPoint::new(b_screen, b_z));
                        rasterizer_points.push(RasterizerPoint::new(c_screen, c_z));
                        // rasterize_colors.push(*color);
                        rasterize_colors.push(Float3::new(0.0, 1.0, 0.0));
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
                        let frac_a = (scene.camera.near - vertex_non_clip.z)
                            / (vertex_a.z - vertex_non_clip.z);
                        let frac_b = (scene.camera.near - vertex_non_clip.z)
                            / (vertex_b.z - vertex_non_clip.z);

                        // New triangle points in view space
                        let clip_vertex_along_edge_a = vertex_non_clip.lerp(vertex_a, frac_a);
                        let clip_vertex_along_edge_b = vertex_non_clip.lerp(vertex_b, frac_b);

                        // New triangle
                        let ((a_screen, a_z), (b_screen, b_z), (c_screen, c_z)) = (
                            view_to_screen(clip_vertex_along_edge_b, &scene.camera, self.size),
                            view_to_screen(vertex_non_clip, &scene.camera, self.size),
                            view_to_screen(clip_vertex_along_edge_a, &scene.camera, self.size),
                        );
                        rasterizer_points.push(RasterizerPoint::new(a_screen, a_z));
                        rasterizer_points.push(RasterizerPoint::new(b_screen, b_z));
                        rasterizer_points.push(RasterizerPoint::new(c_screen, c_z));
                        // rasterize_colors.push(*color);
                        rasterize_colors.push(Float3::new(0.0, 0.0, 1.0));
                    }
                    _ => continue,
                }
            }

            for (chunk, color) in rasterizer_points.chunks_exact(3).zip(rasterize_colors) {
                let (
                    RasterizerPoint {
                        screen_pos: a,
                        depth: a_z,
                    },
                    RasterizerPoint {
                        screen_pos: b,
                        depth: b_z,
                    },
                    RasterizerPoint {
                        screen_pos: c,
                        depth: c_z,
                    },
                ) = (chunk[0], chunk[1], chunk[2]);

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
                            let depths = (1.0 + Float3::new(a_z, b_z, c_z)) * 0.5;
                            let depth = depths.dot(weights);
                            if depth > self.depth_buffer[y * self.width + x] {
                                continue;
                            }

                            self.color_buffer[y * self.width + x] = color;
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
                let c =
                    255 - (self.depth_buffer[y * self.width + x] * 255.0).clamp(0.0, 255.0) as u8;
                bytes[((self.height - 1 - y) * self.width + x) * 4 + 0] = c;
                bytes[((self.height - 1 - y) * self.width + x) * 4 + 1] = c;
                bytes[((self.height - 1 - y) * self.width + x) * 4 + 2] = c;
                bytes[((self.height - 1 - y) * self.width + x) * 4 + 3] = 255;
            }
        }
    }
}

fn model_to_world(vertex: Float3, transform: &Transform) -> Float3 {
    transform.to_world_point(vertex)
}

fn world_to_view(vertex: Float3, camera: &Camera) -> Float3 {
    camera.transform.to_local_point(vertex)
}

fn view_to_screen(vertex: Float3, camera: &Camera, size: Float2) -> (Float2, f32) {
    // Perspective projection
    // From view space to normalized device coordinates
    let top = -camera.near * (camera.fov / 2.0).tan();
    let bottom = -top;
    let right = top * size.x / size.y;
    let left = -right;

    let vertex_persp = Float3::new(
        2.0 * camera.near / (right - left) * vertex.x - vertex.z * (right + left) / (right - left),
        2.0 * camera.near / (top - bottom) * vertex.y - vertex.z * (top + bottom) / (top - bottom),
        (camera.far + camera.near) / (camera.far - camera.near) * vertex.z
            - 2.0 * camera.far * camera.near / (camera.far - camera.near),
    ) / vertex.z;

    // Non-invertible projection onto screen space
    let vertex_screen = Float2::new(
        (vertex_persp.x + 1.0) * 0.5 * size.x,
        (1.0 - (vertex_persp.y + 1.0) * 0.5) * size.y,
    );

    (vertex_screen, vertex_persp.z)
}
