use crate::math::{Float2, Float3};
use crate::texture::Texture;

pub trait Shader {
    fn color(&self, texture_coord: Float2, normal: Float3) -> Float3;
}

pub struct TextureShader {
    pub texture: Texture<Float3>,
}

impl TextureShader {
    pub fn new(texture: Texture<Float3>) -> Self {
        TextureShader { texture }
    }
}

impl Shader for TextureShader {
    fn color(&self, texture_coord: Float2, _normal: Float3) -> Float3 {
        self.texture.sample(texture_coord)
    }
}

pub struct DiffuseShader {
    pub color: Float3,
    pub direction_to_light: Float3,
    pub ambient_factor: f32,
}

impl DiffuseShader {
    pub fn new(color: Float3, direction_to_light: Float3, ambient_factor: f32) -> Self {
        DiffuseShader { color, direction_to_light, ambient_factor }
    }
}

impl Shader for DiffuseShader {
    fn color(&self, _texture_coord: Float2, normal: Float3) -> Float3 {
        let normal = normal.normalized();
        let light_intensity = normal.dot(self.direction_to_light).max(0.0);
        // (normal + 1.0) * 0.5
        self.color * (self.ambient_factor + light_intensity)
    }
}
