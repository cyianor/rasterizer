use crate::math::{Float3, Float4, Float4x4};

/// Transformation of an object in relation to its parent or world space if there is no parent.
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    /// Rotation around upwards vector
    pub yaw: f32,
    /// Rotation around rightwards vector
    pub pitch: f32,
    /// Rotation around forwards vector
    pub roll: f32,
    /// Position of the object
    pub position: Float3,
    /// Scale of the object
    pub scale: Float3,
}

impl Transform {
    /// Create a new transformation from attributes
    pub fn new(yaw: f32, pitch: f32, roll: f32, position: Float3, scale: Float3) -> Self {
        Self {
            yaw,
            pitch,
            roll,
            position,
            scale,
        }
    }

    /// Create a new transformation which is described by a target which is being "looked at"
    /// and an upward vector (often [0, 1, 0]).
    pub fn from_vectors(target: Float3, up: Float3, position: Float3, scale: Float3) -> Self {
        let fwd = (position - target).normalized();
        let right = up.cross(fwd).normalized();
        let up = fwd.cross(right);

        // rows of rotation matrix right, up, fwd
        let yaw = (-fwd.x).atan2(fwd.z);
        let pitch = fwd.y.asin();
        let roll = (-right.y).atan2(up.y);

        Self::new(yaw, pitch, roll, position, scale)
    }

    /// Transform a homogenous point from model space to world space
    pub fn to_world_point(&self, p: Float4) -> Float4 {
        self.world_matrix() * p
    }

    /// Get the homogeneous matrix used to describe the transform from model space to world space
    pub fn world_matrix(&self) -> Float4x4 {
        Float4x4::translation(self.position)
            * Float4x4::scaling(self.scale)
            * self.get_inverse_rotation()
    }

    /// Transform a homogeneous point from world space to model space
    pub fn to_local_point(&self, p: Float4) -> Float4 {
        self.inverse_world_matrix() * p
    }

    /// Get the homogeneous matrix used to describe the transform from world space to model space
    pub fn inverse_world_matrix(&self) -> Float4x4 {
        self.get_rotation()
            * Float4x4::scaling(1.0 / self.scale)
            * Float4x4::translation(-self.position)
    }

    /// Get the homogeneous matrix used to describe the rotation part of the transform from world space to model space
    pub fn get_rotation(&self) -> Float4x4 {
        Float4x4::rotation_z(self.roll)
            * Float4x4::rotation_x(self.pitch)
            * Float4x4::rotation_y(self.yaw)
    }

    /// Get the homogeneous matrix used to describe the rotation part of the transform from model space to world space
    pub fn get_inverse_rotation(&self) -> Float4x4 {
        self.get_rotation().transpose()
    }

    /// Get the basis vectors describing the up, right, and forward vectors in model space
    pub fn get_basis_vectors(&self) -> (Float3, Float3, Float3) {
        let rot = self.get_rotation();

        (rot.r1.xyz(), rot.r2.xyz(), rot.r3.xyz())
    }
}
