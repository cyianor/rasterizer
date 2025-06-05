use crate::math::Float3;
use crate::transform::Transform;

pub struct Camera {
    pub fov: f32,
    pub transform: Transform,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(position: Float3, target: Float3, up: Float3, fov: f32, near: f32, far: f32) -> Self {
        let cam_fwd = (position - target).normalized();
        let cam_right = up.cross(cam_fwd).normalized();
        let cam_up = cam_fwd.cross(cam_right);
        println!("cam_right: {:?}, cam_up: {:?}, cam_fwd: {:?}", cam_right, cam_up, cam_fwd);

        // rows of rotation matrix cam_right, cam_up, cam_fwd
        let yaw = (-cam_fwd.x).atan2(cam_fwd.z);
        let pitch = cam_fwd.y.asin();
        let roll = (-cam_right.y).atan2(cam_up.y);
        // let yaw = (-cam_right.z).atan2(cam_fwd.z);
        // let pitch = cam_up.z.asin();
        // let roll = (-cam_up.x).atan2(cam_up.y);
        
        println!("yaw: {yaw}, pitch: {pitch}, roll: {roll}");

        let transform = Transform::new(yaw, pitch, roll, position, 1.0);
        let (ihat, jhat, khat) = transform.get_basis_vectors();        
        println!("ihat: {:?}, jhat: {:?}, khat: {:?}", ihat, jhat, khat);

        Self {
            fov,
            transform: Transform::new(yaw, pitch, roll, position, 1.0),
            near,
            far,
        }
    }
}
