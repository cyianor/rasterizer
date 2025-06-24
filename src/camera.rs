use crate::math::{Float3, Float4x4};
use crate::transform::Transform;

/// A virtual camera
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    /// Aspect-ratio of camera
    pub aspect_ratio: f32,
    /// Distance to near view plane along negative z-axis (in view space)
    pub near: f32,
    /// Distance to far view plane along negative z-axis (in view space)
    /// Observe that `far < near`.
    pub far: f32,
    /// Distance to top view plane
    pub top: f32,
    /// Distance to bottom view plane
    pub bottom: f32,
    /// Distance to right view plane
    pub right: f32,
    /// Distance to left view plane
    pub left: f32,
    /// Transformation of the camera
    pub transform: Transform,
    /// Projection matrix of the camera
    pub projection: Float4x4,
}

impl Camera {
    /// Create a new camera that looks at target, has an up-vector, a vertical field-of-view (fov)
    /// and aspect ratio for the width.
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
            transform: Transform::from_vectors(target, up, position, Float3::ones()),
            projection,
        }
    }

    /// Create a new camera that looks at target, has an up-vector and with a field-of-view
    /// defined by the width and aspect-ratio.
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
            transform: Transform::from_vectors(target, up, position, Float3::ones()),
            projection,
        }
    }
}
