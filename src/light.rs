use crate::camera::Camera;
use crate::math::Float3;
use crate::texture::Texture;

#[derive(Debug, Clone)]
pub struct SpotLight {
    pub color: Float3,
    pub position: Float3,
    pub target: Float3,
    pub angle: f32,
    pub camera: Camera,
    pub shadow_map: Texture<f32>,
}

impl SpotLight {
    pub fn new(
        color: Float3,
        position: Float3,
        target: Float3,
        angle: f32,
        shadow_map_width: usize,
        shadow_map_height: usize,
    ) -> Self {
        Self {
            color,
            position,
            target,
            angle,
            camera: Camera::new(
                position,
                target,
                Float3::unit_y(),
                angle,
                (shadow_map_width as f32) / (shadow_map_height as f32),
                -0.01,
                -100.0,
            ),
            shadow_map: Texture::new(shadow_map_width, shadow_map_height),
        }
    }
}
