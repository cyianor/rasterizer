use crate::light::SpotLight;
use crate::math::{Float3, Float4, Float4x4};
use crate::texture::Texture;
use crate::render::VertexAttributes;

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
    pub model_world_matrix: Float4x4,
    pub camera_view_proj_matrix: Float4x4,
}

impl ModelShader {
    pub fn new(model_world_matrix: Float4x4, camera_view_proj_matrix: Float4x4) -> Self {
        Self {
            model_world_matrix,
            camera_view_proj_matrix,
        }
    }

    pub fn transform(
        &self,
        vertices: &Vec<Float3>,
        normals: &Vec<Float3>,
    ) -> (Vec<(Float4, u8)>, Vec<Float3>, Vec<Float3>) {
        let world_vertices = vertices
            .iter()
            .map(|v| &self.model_world_matrix * Float4::from_point(*v))
            .collect::<Vec<_>>();

        let vertices = world_vertices
            .iter()
            .map(|v| &self.camera_view_proj_matrix * v)
            .map(|v| (v, culling_bitmask(&v)))
            .collect::<Vec<_>>();

        let vertices_attr = world_vertices
            .iter()
            .map(|v| v.xyz() / v.w)
            .collect::<Vec<_>>();

        let normals = normals
            .iter()
            .map(|n| {
                (&self.model_world_matrix * Float4::from_vector(*n))
                    .xyz()
                    .normalized()
            })
            .collect::<Vec<_>>();

        (vertices, vertices_attr, normals)
    }
}

pub trait Shader {
    fn color(&self, attrs: VertexAttributes) -> Float3;
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
    fn color(&self, attrs: VertexAttributes) -> Float3 {
        self.texture.sample(attrs.uv)
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
    fn color(&self, attrs: VertexAttributes) -> Float3 {
        let normal = attrs.normal.normalized();
        let light_intensity = normal.dot(self.direction_to_light).max(0.0);
        // (normal + 1.0) * 0.5
        self.color * (self.ambient_factor + light_intensity)
    }
}

pub struct DiffuseShaderWithSpotlight {
    pub color: Float3,
    pub direction_to_light: Float3,
    pub ambient_factor: f32,
    pub spotlight: SpotLight,
}

impl DiffuseShaderWithSpotlight {
    pub fn new(
        color: Float3,
        direction_to_light: Float3,
        ambient_factor: f32,
        spotlight: SpotLight,
    ) -> Self {
        DiffuseShaderWithSpotlight {
            color,
            direction_to_light,
            ambient_factor,
            spotlight,
        }
    }
}

impl Shader for DiffuseShaderWithSpotlight {
    fn color(&self, attrs: VertexAttributes) -> Float3 {
        let normal = attrs.normal.normalized();
        let light_intensity = normal.dot(self.direction_to_light).max(0.0);
        let to_light = self.spotlight.position - attrs.vertex;
        let dir_to_light = to_light.normalized();
        let dir_to_target = (self.spotlight.position - self.spotlight.target).normalized();
        let cos_angle_cone = dir_to_light.dot(dir_to_target);
        let spot_intensity = if cos_angle_cone > self.spotlight.angle.cos() {
            normal.dot(dir_to_light).max(0.0) * cos_angle_cone.max(0.0)
        } else {
            0.0
        };

        self.color * light_intensity * self.ambient_factor
            + (1.0 - self.ambient_factor) * self.spotlight.color * spot_intensity
    }
}
