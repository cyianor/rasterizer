use crate::math::{Float2, Float3};
use png::{ColorType, Decoder};
use std::fs::File;

pub struct Texture<T> {
    pub width: usize,
    pub height: usize,
    pub image: Vec<T>,
}

impl Texture<Float3> {
    pub fn from_png(path: &str) -> Texture<Float3> {
        let decoder = Decoder::new(File::open(path).unwrap());
        let mut reader = decoder.read_info().unwrap();
        // Allocate and read to buffer
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).unwrap();
        // Grab bytes of image
        let bytes = buf[..info.buffer_size()].to_owned();

        let image = match info.color_type {
            ColorType::Indexed => {
                if let Some(palette) = &reader.info().palette {
                    bytes
                        .into_iter()
                        .map(|b| {
                            Float3::new(
                                palette[3 * b as usize] as f32 / 255.0,
                                palette[3 * b as usize + 1] as f32 / 255.0,
                                palette[3 * b as usize + 2] as f32 / 255.0,
                            )
                        })
                        .collect::<Vec<_>>()
                } else {
                    panic!("Color type is `Indexed`, but missing palette in {path}")
                }
            }
            _ => {
                panic!("Color type {:?} not supported", info.color_type)
            }
        };

        Texture {
            width: info.width as usize,
            height: info.height as usize,
            image,
        }
    }
}

impl<T: Copy> Texture<T> {
    pub fn new(width: usize, height: usize) -> Texture<T> {
        Texture {
            width,
            height,
            image: Vec::with_capacity(width * height),
        }
    }
    
    pub fn sample(&self, texture_coord: Float2) -> T {
        let x = (texture_coord.x.clamp(0.0, 1.0) * (self.width as f32 - 1.0)).round() as usize;
        let y = (texture_coord.y.clamp(0.0, 1.0) * (self.height as f32 - 1.0)).round() as usize;

        // self.image[x * self.height as usize + y]
        self.image[(self.width as usize - 1 - y) * self.width as usize + x]
    }
}
