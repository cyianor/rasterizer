use crate::math::Float3;
use crate::model::{Model, read_obj_file};
use crate::render::RenderTarget;
use crate::transform::Transform;
use raylib::prelude::*;

use rand::distr::{Distribution, Uniform};

pub struct Camera {
    pub fov: f32,
    pub transform: Transform,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            fov: 60f32.to_radians(),
            transform: Transform::new(0.0, 0.0, Float3::zeros()),
        }
    }
}

pub struct Scene {
    pub camera: Camera,
    pub models: Vec<Model>,
}

impl Scene {
    pub fn new() -> Self {
        let mut scene = Self {
            camera: Camera::new(),
            models: Vec::new(),
        };

        let triangle_points = read_obj_file("models/cube.obj").unwrap();

        let mut rng = rand::rng();
        let uniform_color = Uniform::new(Float3::zeros(), Float3::ones()).unwrap();

        let triangle_colors = (0..triangle_points.len() / 3)
            .map(|_| uniform_color.sample(&mut rng))
            .collect::<Vec<Float3>>();

        let transform = Transform::new(
            0f32.to_radians(),
            0f32.to_radians(),
            Float3::new(0.0, 2.0, 3.0),
        );

        scene
            .models
            .push(Model::new(triangle_points, triangle_colors, transform));

        let triangle_points = read_obj_file("models/suzanne.obj").unwrap();

        let mut rng = rand::rng();
        let uniform_color = Uniform::new(Float3::zeros(), Float3::ones()).unwrap();

        let triangle_colors = (0..triangle_points.len() / 3)
            .map(|_| uniform_color.sample(&mut rng))
            .collect::<Vec<Float3>>();

        let transform = Transform::new(
            0f32.to_radians(),
            0f32.to_radians(),
            Float3::new(0.0, -1.0, 3.0),
        );

        scene
            .models
            .push(Model::new(triangle_points, triangle_colors, transform));

        scene
    }

    pub fn update(&mut self, target: &RenderTarget, rl: &RaylibHandle) {
        let delta_time = rl.get_frame_time();
        // rotate monkey
        self.models[1].transform.yaw += delta_time;

        // move camera
        let cam_speed = 1.5f32;
        let cam_transform = &mut self.camera.transform;
        if rl.is_key_down(KeyboardKey::KEY_W) {
            cam_transform.position.z += cam_speed * delta_time;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            cam_transform.position.z -= cam_speed * delta_time;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            cam_transform.position.x -= cam_speed * delta_time;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            cam_transform.position.x += cam_speed * delta_time;
        }
    }
}
