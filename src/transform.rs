use crate::math::Float3;
// use crate::math::Float4x4;

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

    pub fn to_world_point(&self, p: Float3) -> Float3 {
        let (ihat, jhat, khat) = self.get_basis_vectors();
        transform_vector(ihat, jhat, khat, p) * self.scale + self.position
    }

    pub fn to_local_point(&self, p: Float3) -> Float3 {
        let (ihat, jhat, khat) = self.get_inverse_basis_vectors();
        transform_vector(ihat, jhat, khat, (p - self.position) / self.scale)
    }

    pub fn get_basis_vectors(&self) -> (Float3, Float3, Float3) {
        // Yaw
        let ihat_yaw = Float3::new(self.yaw.cos(), 0.0, self.yaw.sin());
        let jhat_yaw = Float3::new(0.0, 1.0, 0.0);
        let khat_yaw = Float3::new(-self.yaw.sin(), 0.0, self.yaw.cos());
        // let m_yaw = Float4x4::rotation_y(self.yaw);
        // Pitch
        let ihat_pitch = Float3::new(1.0, 0.0, 0.0);
        let jhat_pitch = Float3::new(0.0, self.pitch.cos(), -self.pitch.sin());
        let khat_pitch = Float3::new(0.0, self.pitch.sin(), self.pitch.cos());
        // Roll
        let ihat_roll = Float3::new(self.roll.cos(), -self.roll.sin(), 0.0);
        let jhat_roll = Float3::new(self.roll.sin(), self.roll.cos(), 0.0);
        let khat_roll = Float3::new(0.0, 0.0, 1.0);
        // Yaw, Pitch, and Roll combined
        let ihat = transform_vector(
            ihat_yaw,
            jhat_yaw,
            khat_yaw,
            transform_vector(ihat_pitch, jhat_pitch, khat_pitch, ihat_roll),
        );
        let jhat = transform_vector(
            ihat_yaw,
            jhat_yaw,
            khat_yaw,
            transform_vector(ihat_pitch, jhat_pitch, khat_pitch, jhat_roll),
        );
        let khat = transform_vector(
            ihat_yaw,
            jhat_yaw,
            khat_yaw,
            transform_vector(ihat_pitch, jhat_pitch, khat_pitch, khat_roll),
        );

        (ihat, jhat, khat)
    }

    pub fn get_inverse_basis_vectors(&self) -> (Float3, Float3, Float3) {
        let (ihat, jhat, khat) = self.get_basis_vectors();
        let ihat_inverse = Float3::new(ihat.x, jhat.x, khat.x);
        let jhat_inverse = Float3::new(ihat.y, jhat.y, khat.y);
        let khat_inverse = Float3::new(ihat.z, jhat.z, khat.z);

        (ihat_inverse, jhat_inverse, khat_inverse)
    }
}

fn transform_vector(ihat: Float3, jhat: Float3, khat: Float3, p: Float3) -> Float3 {
    p.x * ihat + p.y * jhat + p.z * khat
}
