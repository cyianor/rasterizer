use crate::math::{Float3, Float4, Float4x4};
use crate::transform::Transform;

pub struct Camera {
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
    pub transform: Transform,
    pub projection: Float4x4,
}

impl Camera {
    pub fn new(position: Float3, target: Float3, up: Float3, fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let cam_fwd = (position - target).normalized();
        let cam_right = up.cross(cam_fwd).normalized();
        let cam_up = cam_fwd.cross(cam_right);

        // rows of rotation matrix cam_right, cam_up, cam_fwd
        let yaw = (-cam_fwd.x).atan2(cam_fwd.z);
        let pitch = cam_fwd.y.asin();
        let roll = (-cam_right.y).atan2(cam_up.y);

        // Perspective projection
        // From view space to normalized device coordinates
        let top = -near * (fov / 2.0).tan();
        let bottom = -top;
        let right = top * aspect_ratio;
        let left = -right;

        let projection = Float4x4::new(
            Float4::new(2.0 * near / (right - left), 0.0, -(right + left) / (right - left), 0.0),
            Float4::new(0.0, 2.0 * near / (top - bottom), -(top + bottom) / (top - bottom), 0.0),
            Float4::new(0.0, 0.0, (far + near) / (far - near), -2.0 * far * near / (far - near)),
            Float4::new(0.0, 0.0, 1.0, 0.0),
        );

        Self {
            fov,
            aspect_ratio,
            near,
            far,
            transform: Transform::new(yaw, pitch, roll, position, Float3::ones()),
            projection
        }
    }
}
