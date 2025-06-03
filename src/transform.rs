use crate::math::Float3;

pub struct Transform {
    pub yaw: f32,   // rotation around upwards vector
    pub pitch: f32, // rotation around rightwards vector
    pub position: Float3,
}

impl Transform {
    pub fn new(yaw: f32, pitch: f32, position: Float3) -> Self {
        Self { yaw, pitch, position }
    }

    pub fn to_world_point(&self, p: Float3) -> Float3 {
        let (ihat, jhat, khat) = self.get_basis_vectors();
        transform_vector(ihat, jhat, khat, p) + self.position
    }

    pub fn to_local_point(&self, p: Float3) -> Float3 {
        p - self.position
    }

    pub fn get_basis_vectors(&self) -> (Float3, Float3, Float3) {
        // Yaw
        let ihat_yaw = Float3::new(self.yaw.cos(), 0.0, self.yaw.sin());
        let jhat_yaw = Float3::new(0.0, 1.0, 0.0);
        let khat_yaw = Float3::new(-self.yaw.sin(), 0.0, self.yaw.cos());
        // Pitch
        let ihat_pitch = Float3::new(1.0, 0.0, 0.0);
        let jhat_pitch = Float3::new(0.0, self.pitch.cos(), -self.pitch.sin());
        let khat_pitch = Float3::new(0.0, self.pitch.sin(), self.pitch.cos());
        // Yaw and Pitch combined
        let ihat = transform_vector(ihat_yaw, jhat_yaw, khat_yaw, ihat_pitch);
        let jhat = transform_vector(ihat_yaw, jhat_yaw, khat_yaw, jhat_pitch);
        let khat = transform_vector(ihat_yaw, jhat_yaw, khat_yaw, khat_pitch);

        (ihat, jhat, khat)
    }
}

fn transform_vector(ihat: Float3, jhat: Float3, khat: Float3, p: Float3) -> Float3 {
    p.x * ihat + p.y * jhat + p.z * khat
}
