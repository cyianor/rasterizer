use crate::math::{Float2, Float3, Float4, Float4x4};
use crate::texture::Texture;

pub struct ShadowMapShader {
    pub transformation: Float4x4,
}

fn culling_bitmask(vertex: &Float4) -> u8 {
    (((vertex.w >= 0.0) as u8) << 6)
        + (((vertex.x + vertex.w >= 0.0) as u8) << 5)
        + (((vertex.x - vertex.w <= 0.0) as u8) << 4)
        + (((vertex.y + vertex.w >= 0.0) as u8) << 3)
        + (((vertex.y - vertex.w <= 0.0) as u8) << 2)
        + (((vertex.z + vertex.w >= 0.0) as u8) << 1)
        + ((vertex.z - vertex.w <= 0.0) as u8)
}

impl ShadowMapShader {
    pub fn new(transformation: Float4x4) -> Self {
        ShadowMapShader { transformation }
    }

    pub fn transform(&self, vertices: Vec<Float3>) -> Vec<(Float4, u8)> {
        vertices
            .into_iter()
            .map(|v| &self.transformation * Float4::from_point(v))
            .map(|v| (v, culling_bitmask(&v)))
            .collect::<Vec<_>>()
    }
}

pub struct ModelShader {
    pub transformation: Float4x4,
    pub rotation: Float4x4,
}

impl ModelShader {
    pub fn new(transformation: Float4x4, rotation: Float4x4) -> Self {
        ModelShader { transformation, rotation }
    }

    pub fn transform(&self, vertices: &Vec<Float3>, normals: &Vec<Float3>) -> (Vec<(Float4, u8)>, Vec<Float3>) {
        let vertices = vertices
            .iter()
            .map(|v| &self.transformation * Float4::from_point(*v))
            .map(|v| (v, culling_bitmask(&v)))
            .collect::<Vec<_>>();

        let normals = normals
            .iter()
            .map(|n| (&self.rotation * Float4::from_vector(*n)).xyz())
            .collect::<Vec<_>>();

        (vertices, normals)
    }
}

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
        DiffuseShader {
            color,
            direction_to_light,
            ambient_factor,
        }
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
