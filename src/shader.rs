use crate::light::SpotLight;
use crate::math::{Float3, Float4, Float4x4};
use crate::render::VertexAttributes;
use crate::texture::Texture;
use rand::distr::{Distribution, Uniform};
use std::cell::RefCell;
use std::rc::Rc;

fn culling_bitmask(vertex: &Float4) -> u8 {
    (((vertex.w >= 0.0) as u8) << 6)
        + (((vertex.x + vertex.w >= 0.0) as u8) << 5)
        + (((vertex.x - vertex.w <= 0.0) as u8) << 4)
        + (((vertex.y + vertex.w >= 0.0) as u8) << 3)
        + (((vertex.y - vertex.w <= 0.0) as u8) << 2)
        + (((vertex.z + vertex.w >= 0.0) as u8) << 1)
        + ((vertex.z - vertex.w <= 0.0) as u8)
}

pub trait VertexShader<I, O> {
    fn transform(&self, input: &I) -> O;
}

/// Vertex shader used on each model during shadow pass
pub struct ShadowPassShader {
    /// Homogeneous matrix describing the transformation from model to world space
    pub model_world_matrix: Float4x4,
    /// Homogeneous matrix describing the transformation from world space to the
    /// light's projected view space
    pub light_view_proj_matrix: Float4x4,
}

/// Input to the shadow pass vertex shader
pub struct ShadowPassShaderInput<'a> {
    /// Vertices in model space
    pub vertices: &'a Vec<Float3>,
}

impl<'a> ShadowPassShaderInput<'a> {
    /// Create a new input to the shadow pass vertex shader
    pub fn new(vertices: &'a Vec<Float3>) -> Self {
        Self { vertices }
    }
}

/// Output from the shadow pass vertex shader
pub struct ShadowPassShaderOutput {
    /// Transformed homogeneous vertices in light's clip space
    pub vertices: Vec<Float4>,
    /// Bit-masks for culling of triangles, stored by vertex
    ///
    /// Stores 7 bits for culling:
    /// - Bit 0 is 1 if `vertex.z - vertex.w <= 0.0`
    /// - Bit 1 is 1 if `vertex.z + vertex.w >= 0.0`
    /// - Bit 2 is 1 if `vertex.y - vertex.w <= 0.0`
    /// - Bit 3 is 1 if `vertex.y + vertex.w >= 0.0`
    /// - Bit 4 is 1 if `vertex.x - vertex.w <= 0.0`
    /// - Bit 5 is 1 if `vertex.x + vertex.w >= 0.0`
    /// - Bit 6 is 1 if `vertex.w >= 0.0`
    ///
    /// A triangle is culled if the bitmasks cb0, cb1, cb2 of the three vertices
    /// fulfill `cb0 & cb1 & cb2 > 0`.
    pub culling_bitmasks: Vec<u8>,
}

impl ShadowPassShader {
    /// Create a new shadow pass vertex shader
    pub fn new(model_world_matrix: Float4x4, light_view_proj_matrix: Float4x4) -> Self {
        Self {
            model_world_matrix,
            light_view_proj_matrix,
        }
    }
}

impl<'a> VertexShader<ShadowPassShaderInput<'a>, ShadowPassShaderOutput> for ShadowPassShader {
    /// Apply vertex shader to vertices in model space
    fn transform(&self, input: &ShadowPassShaderInput<'a>) -> ShadowPassShaderOutput {
        let transformation = self.light_view_proj_matrix * self.model_world_matrix;

        let vertices = input
            .vertices
            .iter()
            .map(|v| &transformation * Float4::from_point(*v))
            .collect::<Vec<_>>();

        let culling_bitmasks = vertices
            .iter()
            .map(|v| culling_bitmask(&v))
            .collect::<Vec<_>>();

        ShadowPassShaderOutput {
            vertices,
            culling_bitmasks,
        }
    }
}

/// Vertex shader used on each model during render pass
pub struct RenderPassShader {
    /// Homogeneous matrix describing the transformation from model to world space
    pub model_world_matrix: Float4x4,
    /// Homogeneous matrix describing the transformation from world space to the
    /// camera's projected view space
    pub camera_view_proj_matrix: Float4x4,
    /// Homogeneous matrix describing the transformation from world space to the
    /// light's projected view space
    pub light_view_proj_matrix: Float4x4,
}

/// Input to the render pass vertex shader
pub struct RenderPassShaderInput<'a, 'b> {
    /// Vertices in model space
    pub vertices: &'a Vec<Float3>,
    /// Normals in model space
    pub normals: &'b Vec<Float3>,
}

impl<'a, 'b> RenderPassShaderInput<'a, 'b> {
    /// Create a new input to the render pass vertex shader
    pub fn new(vertices: &'a Vec<Float3>, normals: &'b Vec<Float3>) -> Self {
        Self { vertices, normals }
    }
}

/// Output of the render pass vertex shader
pub struct RenderPassShaderOutput {
    /// Transformed homogeneous vertices in camera's clip space
    pub vertices: Vec<Float4>,
    /// Bit-masks for culling of triangles, stored by vertex
    ///
    /// Stores 7 bits for culling:
    /// - Bit 0 is 1 if `vertex.z - vertex.w <= 0.0`
    /// - Bit 1 is 1 if `vertex.z + vertex.w >= 0.0`
    /// - Bit 2 is 1 if `vertex.y - vertex.w <= 0.0`
    /// - Bit 3 is 1 if `vertex.y + vertex.w >= 0.0`
    /// - Bit 4 is 1 if `vertex.x - vertex.w <= 0.0`
    /// - Bit 5 is 1 if `vertex.x + vertex.w >= 0.0`
    /// - Bit 6 is 1 if `vertex.w >= 0.0`
    ///
    /// A triangle is culled if the bitmasks cb0, cb1, cb2 of the three vertices
    /// fulfill `cb0 & cb1 & cb2 > 0`.
    pub culling_bitmasks: Vec<u8>,
    /// Transformed homogeneous vertices in light's clip space
    pub light_vertices: Vec<Float4>,
    /// Transformed vertices in world space
    pub vertices_attr: Vec<Float3>,
    /// Transformed normals rotated to world space
    pub normals: Vec<Float3>,
}

impl RenderPassShader {
    /// Create a new render pass vertex shader
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
}

impl<'a, 'b> VertexShader<RenderPassShaderInput<'a, 'b>, RenderPassShaderOutput>
    for RenderPassShader
{
    /// Apply vertex shader to vertices and normals in model space
    fn transform(&self, input: &RenderPassShaderInput<'a, 'b>) -> RenderPassShaderOutput {
        let world_vertices = input
            .vertices
            .iter()
            .map(|v| &self.model_world_matrix * Float4::from_point(*v))
            .collect::<Vec<_>>();

        let vertices = world_vertices
            .iter()
            .map(|v| &self.camera_view_proj_matrix * v)
            .collect::<Vec<_>>();

        let culling_bitmasks = vertices
            .iter()
            .map(|v| culling_bitmask(&v))
            .collect::<Vec<_>>();

        let light_vertices = world_vertices
            .iter()
            .map(|v| &self.light_view_proj_matrix * v)
            .collect::<Vec<_>>();

        let vertices_attr = world_vertices
            .iter()
            .map(|v| v.xyz() / v.w)
            .collect::<Vec<_>>();

        let normals = input
            .normals
            .iter()
            .map(|n| {
                (&self.model_world_matrix * Float4::from_vector(*n))
                    .xyz()
                    .normalized()
            })
            .collect::<Vec<_>>();

        RenderPassShaderOutput {
            vertices,
            culling_bitmasks,
            light_vertices,
            vertices_attr,
            normals,
        }
    }
}

/// Trait describing a pixel shader
pub trait PixelShader {
    /// Given vertex attributes a pixel shader generates a color
    fn color(&self, attrs: VertexAttributes) -> Float3;
}

/// Pixel shader presenting a texture
pub struct TextureShader {
    /// The RGB texture to be sampled in the shader
    pub texture: Texture<Float3>,
}

impl TextureShader {
    /// Create a new texture shader
    pub fn new(texture: Texture<Float3>) -> Self {
        TextureShader { texture }
    }
}

impl PixelShader for TextureShader {
    fn color(&self, attrs: VertexAttributes) -> Float3 {
        self.texture.sample(attrs.uv)
    }
}

/// A diffuse color shader
pub struct DiffuseShader {
    /// Object color
    pub color: Float3,
    /// Direction to surrounding light
    pub direction_to_light: Float3,
    /// Intensity of ambient light
    pub ambient_factor: f32,
}

impl DiffuseShader {
    /// Create a new diffuse shader
    pub fn new(color: Float3, direction_to_light: Float3, ambient_factor: f32) -> Self {
        DiffuseShader {
            color,
            direction_to_light: direction_to_light.normalized(),
            ambient_factor,
        }
    }
}

impl PixelShader for DiffuseShader {
    fn color(&self, attrs: VertexAttributes) -> Float3 {
        let normal = attrs.normal.normalized();
        let light_intensity = normal.dot(self.direction_to_light).max(0.0);
        // (normal + 1.0) * 0.5
        self.color * (self.ambient_factor + light_intensity)
    }
}

/// A diffuse color shader supporting one spotlight
pub struct DiffuseShaderWithSpotlight {
    /// Color of the object
    pub color: Float3,
    /// Direction to surrounding light
    pub direction_to_light: Float3,
    /// Intensity of ambient light
    pub ambient_factor: f32,
    /// Spotlight
    pub spotlight: Rc<RefCell<SpotLight>>,
}

impl DiffuseShaderWithSpotlight {
    /// Create a new shader
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

impl PixelShader for DiffuseShaderWithSpotlight {
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
        let bias = (0.5 * (1.0 - normal.dot(dir_to_light))).max(0.1);
        let bias = 0.5;

        let percentage_in_light = if light_fragment.x >= 0.0
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
            // let xsign = [1.0, -1.0];
            // let ysign = [1.0, -1.0];
            // let xsign = [1.0, -0.0, -1.0];
            // let ysign = [1.0, -0.0, -1.0];
            let xsign = [1.5, -0.5, -1.5];
            let ysign = [1.5, -0.5, -1.5];
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

        Float3::new(percentage_in_light, percentage_in_light, percentage_in_light)

        // let color = self.color * 0.1
        //     + self.color * light_intensity * 0.2
        //     + self.color * spotlight.color * spot_intensity * percentage_in_light * 3.0;
        // // Gamma-correction
        // color.powf(1.0 / 2.2)
    }
}
