use rand::distr::uniform::UniformSampler;
use rand::distr::{Distribution, Uniform};

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
        // Goal here: avoid shadow acne without getting peter panning
        // TODO: Find better way to choose bias
        let bias = (0.5 * (1.0 - normal.dot(dir_to_light))).max(0.2);
        // let bias = 0.0;

        let percentage_in_shadow = if light_fragment.x >= 0.0
            && light_fragment.x <= 1.0
            && light_fragment.y >= 0.0
            && light_fragment.y <= 1.0
            && light_fragment.z >= 0.0
            && light_fragment.z <= 1.0
        {
            // Percentage-closer filtering
            let linear_frag_z = linearize_depth(
                light_fragment.z,
                spotlight.camera.near,
                spotlight.camera.far,
            );

            let sampler = Uniform::new(0.0, 1.0).unwrap();
            let mut rng = rand::rng();
            let width = spotlight.shadow_map.width as f32 - 1.0;
            let height = spotlight.shadow_map.height as f32 - 1.0;
            // let xsign = [0.0];
            // let ysign = [0.0];
            let xsign = [1.0, -1.0];
            let ysign = [1.0, -1.0];
            // let xsign = [1.5, -0.5, -1.5];
            // let ysign = [1.5, -0.5, -1.5];
            let repeats = 1;
            let mut p = 0.0;
            for _ in 0..repeats {
                for xs in xsign {
                    for ys in ysign {
                        let x = (light_fragment.x * width + xs * sampler.sample(&mut rng))
                            .clamp(0.0, width) as usize;
                        let y = ((1.0 - light_fragment.y) * height + ys * sampler.sample(&mut rng))
                            .clamp(0.0, height) as usize;

                        p += (linear_frag_z + bias
                            >= linearize_depth(
                                spotlight.shadow_map.image[y * spotlight.shadow_map.width + x],
                                spotlight.camera.near,
                                spotlight.camera.far,
                            )) as i32 as f32;
                    }
                }
            }

            p / (repeats as f32 * (xsign.len() * xsign.len()) as f32)
        } else {
            0.0
        };

        let spot_intensity = if dir_to_light.dot(dir_to_target) > spotlight.angle.cos() {
            normal.dot(dir_to_light).max(0.0) / to_light.norm()
        } else {
            0.0
        };

        // Float3::new(percentage_in_shadow, percentage_in_shadow, percentage_in_shadow)

        let color = self.color * 0.1
            + self.color * light_intensity * 0.2
            + self.color * spotlight.color * spot_intensity * percentage_in_shadow * 2.5;
        // Gamma-correction
        color.powf(1.0 / 2.2)
    }
}
