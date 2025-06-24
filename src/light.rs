use crate::camera::Camera;
use crate::math::Float3;
use crate::texture::Texture;

/// A cone-like spotlight
pub struct SpotLight {
    /// Color of the emitted light
    pub color: Float3,
    /// Position of the spotlight in world space
    pub position: Float3,
    /// Point targeted by the spotlight in world space
    pub target: Float3,
    /// Angle of the spotlight's cone
    pub angle: f32,
    /// Camera used for producing a shadow map (uses perspective projection)
    pub camera: Camera,
    /// Shadow map as a depth texture
    pub shadow_map: Texture<f32>,
}

impl SpotLight {
    /// Create a new cone-like spotlight.
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
                2.0 * angle,
                (shadow_map_width as f32) / (shadow_map_height as f32),
                -1.0,
                -100.0,
            ),
            shadow_map: Texture::new(shadow_map_width, shadow_map_height),
        }
    }
}
