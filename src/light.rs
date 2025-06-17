use crate::math::Float3;
// use crate::texture::Texture;

#[derive(Debug, Clone, Copy)]
pub struct SpotLight {
    pub color: Float3,
    pub position: Float3,
    pub target: Float3,
    pub angle: f32,
    // pub shadow_map: Texture<f32>,
}

impl SpotLight {
    pub fn new(color: Float3, position: Float3, target: Float3, angle: f32) -> Self { //, shadow_map_width: usize, shadow_map_height: usize) -> SpotLight {
        Self {
            color,
            position,
            target,
            angle,
            // shadow_map: Texture::new(shadow_map_width, shadow_map_height),
        }
    }
}
