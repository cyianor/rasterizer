use crate::math::{Float2, Float3};
use png::{ColorType, Decoder};
use std::fs::File;

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub image: Vec<Float3>,
}

impl Texture {
    pub fn from_png(path: &str) -> Texture {
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

        // let img = ImageReader::open(path).unwrap().decode().unwrap();

        // Texture {
        //     width: img.width(),
        //     height: img.height(),
        //     image: img.
        //     image: img
        //         .as_rgb32f()
        //         .unwrap()
        //         .enumerate_pixels()
        //         .map(|(x, y, c)| Float3::new(c.0[0], c.0[1], c.0[2]))
        //         .collect::<Vec<_>>(),
        // }

        Texture {
            width: info.width,
            height: info.height,
            image,
        }
    }

    pub fn sample(&self, texture_coord: Float2) -> Float3 {
        let x = (texture_coord.x.clamp(0.0, 1.0) * (self.width as f32 - 1.0)).round() as usize;
        let y = (texture_coord.y.clamp(0.0, 1.0) * (self.height as f32 - 1.0)).round() as usize;

        // self.image[x * self.height as usize + y]
        self.image[(self.width as usize - 1 - y) * self.width as usize + x]
    }
}
