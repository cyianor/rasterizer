use crate::math::{Float3, Float4, Float4x4};

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub yaw: f32,   // rotation around upwards vector
    pub pitch: f32, // rotation around rightwards vector
    pub roll: f32,  // rotation around forwards vector
    pub position: Float3,
    pub scale: Float3,
}

impl Transform {
    pub fn new(yaw: f32, pitch: f32, roll: f32, position: Float3, scale: Float3) -> Self {
        Self {
            yaw,
            pitch,
            roll,
            position,
            scale,
        }
    }

    pub fn from_vectors(position: Float3, target: Float3, up: Float3, scale: Float3) -> Self {
        let fwd = (position - target).normalized();
        let right = up.cross(fwd).normalized();
        let up = fwd.cross(right);

        // rows of rotation matrix right, up, fwd
        let yaw = (-fwd.x).atan2(fwd.z);
        let pitch = fwd.y.asin();
        let roll = (-right.y).atan2(up.y);

        Self::new(yaw, pitch, roll, position, scale)
    }

    pub fn to_world_point(&self, p: Float4) -> Float4 {
        // let rot = self.get_inverse_rotation();
        // (rot * p) * Float4::from_point(self.scale) + Float4::from_vector(self.position)
        self.world_matrix() * p
    }

    pub fn world_matrix(&self) -> Float4x4 {
        Float4x4::translation(self.position)
            * Float4x4::scaling(self.scale)
            * self.get_inverse_rotation()
    }

    pub fn to_local_point(&self, p: Float4) -> Float4 {
        // let rot = self.get_rotation();
        // rot * ((p - Float4::from_vector(self.position)) / Float4::from_point(self.scale))
        self.inverse_world_matrix() * p
    }

    pub fn inverse_world_matrix(&self) -> Float4x4 {
        self.get_rotation()
            * Float4x4::scaling(1.0 / self.scale)
            * Float4x4::translation(-self.position)
    }

    pub fn get_rotation(&self) -> Float4x4 {
        Float4x4::rotation_z(self.roll)
            * Float4x4::rotation_x(self.pitch)
            * Float4x4::rotation_y(self.yaw)
    }

    pub fn get_inverse_rotation(&self) -> Float4x4 {
        self.get_rotation().transpose()
    }

    pub fn get_basis_vectors(&self) -> (Float3, Float3, Float3) {
        let rot = self.get_rotation();

        (rot.r1.xyz(), rot.r2.xyz(), rot.r3.xyz())
    }

    pub fn get_inverse_basis_vectors(&self) -> (Float3, Float3, Float3) {
        let rot = self.get_inverse_rotation();

        (rot.r1.xyz(), rot.r2.xyz(), rot.r3.xyz())
    }
}
