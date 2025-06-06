use crate::camera::Camera;
use crate::math::Float3;
use crate::model::{Model, read_obj_file};
use crate::render::RenderTarget;
use crate::transform::Transform;
use rand::distr::{Distribution, Uniform};
use raylib::prelude::*;

pub struct Scene {
    pub camera: Camera,
    pub models: Vec<Model>,
    pub total_frame_time: f32,
    pub average_frame_time: f32,
    pub frame_counter: i32,
    pub last_frame_counter: i32,
}

impl Scene {
    pub fn new() -> Self {
        let mut scene = Self {
            camera: Camera::new(
                Float3::new(0.0, 1.0, -10.0),
                Float3::zeros(),
                Float3::new(0.0, 1.0, 0.0),
                60f32.to_radians(),
                -1.0,
                -50.0,
            ),
            models: Vec::new(),
            total_frame_time: 0.0,
            average_frame_time: 0.0,
            frame_counter: 0,
            last_frame_counter: 0,
        };

        let triangle_points = read_obj_file("models/cube.obj").unwrap();

        let mut rng = rand::rng();
        let uniform_color = Uniform::new(Float3::zeros(), Float3::ones()).unwrap();

        let triangle_colors = (0..triangle_points.len() / 3)
            .map(|_| uniform_color.sample(&mut rng))
            .collect::<Vec<Float3>>();

        let transform = Transform::new(0.0, 0.0, 0.0, Float3::new(5.0, 0.0, 0.0), 1.0);

        scene
            .models
            .push(Model::new(triangle_points, triangle_colors, transform));

        let triangle_points = read_obj_file("models/Dragon_8K.obj").unwrap();

        let mut rng = rand::rng();
        let uniform_color = Uniform::new(Float3::zeros(), Float3::ones()).unwrap();

        let triangle_colors = (0..triangle_points.len() / 3)
            .map(|_| uniform_color.sample(&mut rng))
            .collect::<Vec<Float3>>();

        let transform = Transform::new(0.0, 0.0, 0.0, Float3::new(0.0, 0.0, 0.0), 5.0);

        scene
            .models
            .push(Model::new(triangle_points, triangle_colors, transform));

        scene
    }

    pub fn update(&mut self, target: &RenderTarget, rl: &RaylibHandle) {
        let delta_time = rl.get_frame_time();
        self.total_frame_time += delta_time;
        self.frame_counter += 1;
        if self.total_frame_time > 1.0 {
            self.average_frame_time = self.total_frame_time / self.frame_counter as f32;
            self.total_frame_time = 0.0;
            self.last_frame_counter = self.frame_counter;
            self.frame_counter = 0;
        }
        // rotate cube
        self.models[0].transform.yaw += delta_time;

        // rotate camera with mouse
        const MOUSE_SENSITIVITY: f32 = 2.0;
        let cam_transform = &mut self.camera.transform;
        let mouse_delta = rl.get_mouse_delta() / target.width as f32 * MOUSE_SENSITIVITY;
        cam_transform.pitch =
            (cam_transform.pitch + mouse_delta.y).clamp((-89f32).to_radians(), 89f32.to_radians());
        cam_transform.yaw += mouse_delta.x;

        // move camera
        const CAM_SPEED: f32 = 5.0;
        let mut move_delta = Float3::zeros();
        let (cam_right, _, cam_fwd) = cam_transform.get_basis_vectors();

        if rl.is_key_down(KeyboardKey::KEY_W) {
            move_delta -= cam_fwd;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            move_delta += cam_fwd;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            move_delta -= cam_right;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            move_delta += cam_right;
        }

        cam_transform.position += move_delta.normalized() * CAM_SPEED * delta_time;
        cam_transform.position.y = 1.0;
    }
}
