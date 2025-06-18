use crate::light::SpotLight;
use crate::math::{Float3, Float4, Float4x4};
use crate::render::VertexAttributes;
use crate::texture::Texture;
use std::cell::RefCell;
use std::rc::Rc;

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
    pub light_view_proj_matrix: Float4x4,
}

pub struct ModelShaderOutput {
    pub vertices: Vec<(Float4, u8)>,
    pub light_vertices: Vec<(Float4, u8)>,
    pub vertices_attr: Vec<Float3>,
    pub light_vertices_attr: Vec<Float4>,
    pub normals: Vec<Float3>,
}

impl ModelShader {
    pub fn new(
        model_world_matrix: Float4x4,
        camera_view_proj_matrix: Float4x4,
        light_view_proj_matrix: Float4x4,
    ) -> Self {
        Self {
            model_world_matrix,
            camera_view_proj_matrix,
            light_view_proj_matrix,
        }
    }

    pub fn transform(&self, vertices: &Vec<Float3>, normals: &Vec<Float3>) -> ModelShaderOutput {
        let world_vertices = vertices
            .iter()
            .map(|v| &self.model_world_matrix * Float4::from_point(*v))
            .collect::<Vec<_>>();

        let vertices = world_vertices
            .iter()
            .map(|v| &self.camera_view_proj_matrix * v)
            .map(|v| (v, culling_bitmask(&v)))
            .collect::<Vec<_>>();

        let light_vertices = world_vertices
            .iter()
            .map(|v| &self.light_view_proj_matrix * v)
            .map(|v| (v, culling_bitmask(&v)))
            .collect::<Vec<_>>();

        let vertices_attr = world_vertices
            .iter()
            .map(|v| v.xyz() / v.w)
            .collect::<Vec<_>>();

        let light_vertices_attr = light_vertices
            .iter()
            .map(|v| v.0) // v.0.xyz() / v.0.w)
            .collect::<Vec<_>>();

        let normals = normals
            .iter()
            .map(|n| {
                (&self.model_world_matrix * Float4::from_vector(*n))
                    .xyz()
                    .normalized()
            })
            .collect::<Vec<_>>();

        ModelShaderOutput {
            vertices,
            light_vertices,
            vertices_attr,
            light_vertices_attr,
            normals,
        }
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
    pub spotlight: Rc<RefCell<SpotLight>>,
}

impl DiffuseShaderWithSpotlight {
    pub fn new(
        color: Float3,
        direction_to_light: Float3,
        ambient_factor: f32,
        spotlight: Rc<RefCell<SpotLight>>,
    ) -> Self {
        DiffuseShaderWithSpotlight {
            color,
            direction_to_light,
            ambient_factor,
            spotlight,
        }
    }
}

fn linearize_depth(depth: f32, near: f32, far: f32) -> f32 {
    2.0 * far * near / (far + near - (2.0 * depth - 1.0) * (far - near))
}

impl Shader for DiffuseShaderWithSpotlight {
    fn color(&self, attrs: VertexAttributes) -> Float3 {
        let spotlight = self.spotlight.borrow();

        let normal = attrs.normal.normalized();
        let light_intensity = normal.dot(self.direction_to_light).max(0.0);
        let to_light = spotlight.position - attrs.vertex;
        let dir_to_light = to_light.normalized();
        let dir_to_target = (spotlight.position - spotlight.target).normalized();
        let light_fragment = attrs.light_vertex.xyz() / attrs.light_vertex.w * 0.5 + 0.5;
        let (current_depth, closest_depth) = if light_fragment.x >= 0.0
            && light_fragment.x <= 1.0
            && light_fragment.y >= 0.0
            && light_fragment.y <= 1.0
            && light_fragment.z >= 0.0
            && light_fragment.z <= 1.0
        {
            let x = (light_fragment.x * spotlight.shadow_map.width as f32)
                .clamp(0.0, spotlight.shadow_map.width as f32 - 1.0) as usize;
            let y = ((1.0 - light_fragment.y) * spotlight.shadow_map.height as f32)
                .clamp(0.0, spotlight.shadow_map.height as f32 - 1.0) as usize;

            (
                linearize_depth(
                    light_fragment.z,
                    spotlight.camera.near,
                    spotlight.camera.far,
                ),
                linearize_depth(
                    spotlight.shadow_map.image[y * spotlight.shadow_map.width + x],
                    spotlight.camera.near,
                    spotlight.camera.far,
                ),
            )
        } else {
            (0.0, f32::NEG_INFINITY)
        };

        // Goal here: avoid shadow acne without getting peter panning
        // TODO: Find better way to choose bias
        let bias = (0.5 * (1.0 - normal.dot(dir_to_light))).max(0.05);
        // let bias = 0.05;

        let spot_intensity = if dir_to_light.dot(dir_to_target) > spotlight.angle.cos()
            && current_depth + bias >= closest_depth
        {
            normal.dot(dir_to_light).max(0.0) / to_light.norm()
        } else {
            0.0
        };

        let color = self.color * 0.1
            + self.color * light_intensity * 0.2
            + self.color * spotlight.color * spot_intensity * 2.5;
        // Gamma-correction
        color.powf(1.0 / 2.2)
    }
}
