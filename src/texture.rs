use crate::math::{Float2, Float3};
use png::{ColorType, Decoder};
use std::fs::File;

/// A texture of generic type `T`
#[derive(Debug, Clone)]
pub struct Texture<T>
where
    T: Copy + Default,
{
    /// Width of the texture
    pub width: usize,
    /// Height of the texture
    pub height: usize,
    /// Image data
    pub image: Vec<T>,
}

impl Texture<Float3> {
    /// Load color-texture from PNG file.
    ///
    /// Colors are represented as [Float3's](crate::math::Float3) with each
    /// component representing red, green, or blue in the interval [0.0, 1.0].
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

impl<T> Texture<T>
where
    T: Copy + Default,
{
    /// Create a new texture
    pub fn new(width: usize, height: usize) -> Texture<T> {
        let mut image: Vec<T> = Vec::new();
        image.resize(width * height * size_of::<T>(), T::default());

        Texture {
            width,
            height,
            image,
        }
    }

    /// Sample from the texture at coordinates (u, v) in [0, 1] x [0, 1]
    /// 
    /// Note that y is inverted from 0 to 1 to 1 to 0 instead.
    pub fn sample(&self, texture_coord: Float2) -> T {
        let x = (texture_coord.x.clamp(0.0, 1.0) * (self.width as f32 - 1.0)).floor() as usize;
        let y = ((1.0 - texture_coord.y).clamp(0.0, 1.0) * (self.height as f32 - 1.0)).floor() as usize;

        self.image[y * self.width + x]
    }
}
