use crate::math::{Float2, Float3, point_in_triangle};

pub struct RenderTarget {
    pub width: usize,
    pub height: usize,
    pub size: Float2,
    pub buf: Vec<Float3>,
}

impl RenderTarget {
    pub fn new(width: usize, height: usize) -> Self {
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
            buf: buf,
        }
    }

    pub fn clear(&mut self, color: Float3) {
        self.buf.fill(color);
    }

    pub fn render(&mut self, points: Vec<Float2>, colors: Vec<Float3>) {
        for (chunk, color) in points.chunks_exact(3).zip(colors) {
            let (a, b, c) = (chunk[0], chunk[1], chunk[2]);

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
                    if point_in_triangle(
                        chunk[0],
                        chunk[1],
                        chunk[2],
                        Float2 {
                            x: x as f32,
                            y: y as f32,
                        },
                    ) {
                        self.buf[y * self.width + x] = color;
                    }
                }
            }
        }
    }
}
