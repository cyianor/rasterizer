use crate::math::{Float3, Float4x4};
use crate::transform::Transform;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
    pub transform: Transform,
    pub projection: Float4x4,
}

impl Camera {
    pub fn new(
        position: Float3,
        target: Float3,
        up: Float3,
        fov: f32,
        aspect_ratio: f32,
        near: f32,
        far: f32,
    ) -> Self {
        // Perspective projection
        // From view space to normalized device coordinates
        let top = -near * (fov / 2.0).tan();
        let bottom = -top;
        let right = top * aspect_ratio;
        let left = -right;

        let projection = Float4x4::perspective_projection(near, far, left, right, top, bottom);

        Self {
            fov,
            aspect_ratio,
            near,
            far,
            transform: Transform::from_vectors(position, target, up, Float3::ones()),
            projection,
        }
    }
}
