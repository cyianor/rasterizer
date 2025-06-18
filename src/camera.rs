use crate::math::{Float3, Float4x4};
use crate::transform::Transform;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
    pub top: f32,
    pub bottom: f32,
    pub right: f32,
    pub left: f32,
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
            aspect_ratio,
            near,
            far,
            top,
            bottom,
            right,
            left,
            transform: Transform::from_vectors(position, target, up, Float3::ones()),
            projection,
        }
    }

    pub fn from_dimensions(
        position: Float3,
        target: Float3,
        up: Float3,
        width: f32,
        aspect_ratio: f32,
        near: f32,
        far: f32,
    ) -> Self {
        // Perspective projection
        // From view space to normalized device coordinates
        let height = width / aspect_ratio;
        let top = height / 2.0;
        let bottom = -height / 2.0;
        let right = width / 2.0;
        let left = -width / 2.0;

        let projection = Float4x4::perspective_projection(near, far, left, right, top, bottom);

        Self {
            aspect_ratio,
            near,
            far,
            top,
            bottom,
            right,
            left,
            transform: Transform::from_vectors(position, target, up, Float3::ones()),
            projection,
        }
    }
}
