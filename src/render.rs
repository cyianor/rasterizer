use crate::math::{Float2, Float3, point_in_triangle};
use crate::model::Model;
use crate::transform::Transform;

pub struct RenderTarget {
    pub width: usize,
    pub height: usize,
    pub size: Float2,
    pub fov: f32,
    pub color_buffer: Vec<Float3>,
}

impl RenderTarget {
    pub fn new(width: usize, height: usize, fov: f32) -> Self {
        let mut buf: Vec<Float3> = Vec::new();
        buf.resize(
            width * height,
            Float3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        );

        Self {
            width,
            height,
            size: Float2 {
                x: width as f32,
                y: height as f32,
            },
            fov,
            color_buffer: buf,
        }
    }

    pub fn clear(&mut self, color: Float3) {
        self.color_buffer.fill(color);
    }

    pub fn render(&mut self, model: Model) {
        for (chunk, color) in model
            .triangle_points
            .chunks_exact(3)
            .zip(model.triangle_colors)
        {
            let (a, b, c) = (
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
                    if point_in_triangle(a, b, c, Float2::new(x as f32, y as f32)) {
                        self.color_buffer[y * self.width + x] = color;
                    }
                }
            }
        }
    }
}

fn world_to_screen(vertex: Float3, transform: Transform, size: Float2, fov: f32) -> Float2 {
    let vertex_world = transform.to_world_point(vertex);

    let screen_height_world: f32 = (fov / 2.0).tan() * 2.0;
    let pixels_per_world_unit = size.y / screen_height_world / vertex_world.z;

    let pixel_offset = Float2::new(vertex_world.x, vertex_world.y) * pixels_per_world_unit;

    size / 2.0 + pixel_offset
}
