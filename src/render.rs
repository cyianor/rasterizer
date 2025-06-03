use core::f32;

use crate::math::{Float2, Float3, point_in_triangle};
use crate::scene::Scene;
use crate::transform::Transform;

pub struct RenderTarget {
    pub width: usize,
    pub height: usize,
    pub size: Float2,
    pub fov: f32,
    pub color_buffer: Vec<Float3>,
    pub depth_buffer: Vec<f32>,
}

impl RenderTarget {
    pub fn new(width: usize, height: usize, fov: f32) -> Self {
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
            fov,
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
                let ((a, a_z), (b, b_z), (c, c_z)) = (
                    world_to_screen(chunk[0], model.transform, self.size, self.fov),
                    world_to_screen(chunk[1], model.transform, self.size, self.fov),
                    world_to_screen(chunk[2], model.transform, self.size, self.fov),
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
                            let depths = Float3::new(a_z, b_z, c_z);
                            let depth = depths.dot(weights);
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

    pub fn color_buffer_to_byte_array(&self, bytes: &mut Vec<u8>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.color_buffer[y * self.width + x] * 255.0;
                bytes[(y * self.width + x) * 4 + 0] = c.x.clamp(0.0, 255.0) as u8;
                bytes[(y * self.width + x) * 4 + 1] = c.y.clamp(0.0, 255.0) as u8;
                bytes[(y * self.width + x) * 4 + 2] = c.z.clamp(0.0, 255.0) as u8;
                bytes[(y * self.width + x) * 4 + 3] = 255;
            }
        }
    }
}

fn world_to_screen(vertex: Float3, transform: Transform, size: Float2, fov: f32) -> (Float2, f32) {
    let vertex_world = transform.to_world_point(vertex);

    let screen_height_world: f32 = (fov / 2.0).tan() * 2.0;
    let pixels_per_world_unit = size.y / screen_height_world / vertex_world.z;

    let pixel_offset = Float2::new(vertex_world.x, vertex_world.y) * pixels_per_world_unit;
    let vertex_screen = size / 2.0 + pixel_offset;

    (vertex_screen, vertex_world.z)
}
