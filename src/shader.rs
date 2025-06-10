use crate::math::{Float2, Float3};
use crate::texture::Texture;

pub trait Shader {
    fn color(&self, texture_coord: Option<Float2>, normal: Option<Float3>) -> Float3;
}

pub struct TextureShader {
    pub texture: Texture,
}

impl TextureShader {
    pub fn new(texture: Texture) -> Self {
        TextureShader { texture }
    }
}

impl Shader for TextureShader {
    fn color(&self, texture_coord: Option<Float2>, _normal: Option<Float3>) -> Float3 {
        if let Some(uv) = texture_coord {
            self.texture.sample(uv)
        } else {
            Float3::new(252.0, 15.0, 192.0) / 255.0
        }
    }
}

pub struct DiffuseShader {
    pub color: Float3,
}

impl DiffuseShader {
    pub fn new(color: Float3) -> Self {
        DiffuseShader { color }
    }
}

impl Shader for DiffuseShader {
    fn color(&self, _texture_coord: Option<Float2>, _normal: Option<Float3>) -> Float3 {
        self.color
    }
}
