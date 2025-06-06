use core::f32;

use crate::camera::Camera;
use crate::math::{Float2, Float3, point_in_triangle};
use crate::scene::Scene;
use crate::transform::Transform;

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
            for (chunk, color) in model
                .triangle_points
                .chunks_exact(3)
                .zip(model.triangle_colors.iter())
            {
                if let (Some((a, a_z)), Some((b, b_z)), Some((c, c_z))) = (
                    to_screen(chunk[0], &model.transform, &scene.camera, self.size),
                    to_screen(chunk[1], &model.transform, &scene.camera, self.size),
                    to_screen(chunk[2], &model.transform, &scene.camera, self.size),
                ) {
                    if a_z < -1.0 || a_z > 1.0 || b_z < -1.0 || b_z > 1.0 || c_z < -1.0 || c_z > 1.0
                    {
                        continue;
                    }

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
                                let depth = 1.0 / (1.0 / depths).dot(weights);
                                if depth > self.depth_buffer[y * self.width + x] {
                                    continue;
                                }

                                self.color_buffer[y * self.width + x] = *color;
                                self.depth_buffer[y * self.width + x] = depth;
                            }
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

fn to_screen(
    vertex: Float3,
    transform: &Transform,
    camera: &Camera,
    size: Float2,
) -> Option<(Float2, f32)> {
    // From model space to world space
    let vertex_world = transform.to_world_point(vertex);
    // From world space to view space
    let vertex_view = camera.transform.to_local_point(vertex_world);

    // Discard points behind camera.
    if vertex_view.z >= 0.0 {
        return None;
    }

    // Perspective projection
    // From view space to normalized device coordinates
    let top = -camera.near * (camera.fov / 2.0).tan();
    let bottom = -top;
    let right = top * size.x / size.y;
    let left = -right;

    let vertex_persp = Float3::new(
        2.0 * camera.near / (right - left) * vertex_view.x
            - vertex_view.z * (right + left) / (right - left),
        2.0 * camera.near / (top - bottom) * vertex_view.y
            - vertex_view.z * (top + bottom) / (top - bottom),
        (camera.far + camera.near) / (camera.far - camera.near) * vertex_view.z
            - 2.0 * camera.far * camera.near / (camera.far - camera.near),
    ) / vertex_view.z;

    // Non-invertible projection onto screen
    let vertex_screen = Float2::new(
        ((vertex_persp.x + 1.0) * 0.5 * size.x).clamp(0.0, size.x - 1.0),
        ((1.0 - (vertex_persp.y + 1.0) * 0.5) * size.y).clamp(0.0, size.y - 1.0),
    );

    Some((vertex_screen, vertex_persp.z))
}
