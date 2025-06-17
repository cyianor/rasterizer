use crate::camera::Camera;
use crate::light::SpotLight;
use crate::math::Float3;
use crate::model::{Model, read_obj_file};
use crate::render::RenderTarget;
use crate::shader::{DiffuseShader, DiffuseShaderWithSpotlight, TextureShader};
use crate::texture::Texture;
use crate::transform::Transform;
use raylib::RaylibHandle;
use raylib::ffi::KeyboardKey;

pub struct Scene {
    pub camera: Camera,
    pub models: Vec<Model>,
    pub total_frame_time: f32,
    pub average_frame_time: f32,
    pub frame_counter: i32,
    pub last_frame_counter: i32,
}

impl Scene {
    pub fn new(aspect_ratio: f32) -> Self {
        let mut scene = Self {
            camera: Camera::new(
                Float3::new(0.0, 2.0, 20.0),
                Float3::zeros(),
                Float3::new(0.0, 1.0, 0.0),
                60f32.to_radians(),
                aspect_ratio,
                -1.0,
                -50.0,
            ),
            models: Vec::new(),
            total_frame_time: 0.0,
            average_frame_time: 0.0,
            frame_counter: 0,
            last_frame_counter: 0,
        };

        let ambient_factor = 0.5f32;
        let direction_to_light = Float3::new(1.0, 1.0, 0.0).normalized();
        let spotlight = SpotLight::new(
            Float3::new(1.0, 1.0, 1.0),
            Float3::new(-5.0, 10.0, 0.0),
            Float3::new(-2.0, 0.0, 1.0),
            30f32.to_radians(),
            128,
            128,
        );

        let (
            vertices,
            vertex_indices,
            texture_coords,
            texture_coord_indices,
            normals,
            normal_indices,
        ) = read_obj_file("models/cube.obj", true, true).unwrap();

        let transform = Transform::new(0.0, 0.0, 0.0, Float3::new(5.0, 1.0, 0.0), Float3::ones());

        let shader = DiffuseShaderWithSpotlight::new(
            Float3::new(1.0, 0.0, 0.0),
            direction_to_light,
            ambient_factor,
            spotlight.clone(),
        );

        scene.models.push(Model::new(
            vertices,
            vertex_indices,
            texture_coords,
            texture_coord_indices,
            normals,
            normal_indices,
            transform,
            Box::new(shader),
        ));

        let (
            vertices,
            vertex_indices,
            texture_coords,
            texture_coord_indices,
            normals,
            normal_indices,
        ) = read_obj_file("models/dragon.obj", true, true).unwrap();

        let transform = Transform::new(0.0, 0.0, 0.0, Float3::new(0.0, 4.0, 0.0), Float3::ones());

        let shader = DiffuseShaderWithSpotlight::new(
            Float3::new(0.0, 1.0, 0.0),
            direction_to_light,
            ambient_factor,
            spotlight.clone(),
        );

        scene.models.push(Model::new(
            vertices,
            vertex_indices,
            texture_coords,
            texture_coord_indices,
            normals,
            normal_indices,
            transform,
            Box::new(shader),
        ));

        let (
            vertices,
            vertex_indices,
            texture_coords,
            texture_coord_indices,
            normals,
            normal_indices,
        ) = read_obj_file("models/floor.obj", true, true).unwrap();

        let transform = Transform::new(0.0, 0.0, 0.0, Float3::new(0.0, 0.0, 0.0), Float3::ones());

        // let texture = Texture::from_png("models/checker-map_tho.png");
        // let shader = TextureShader::new(texture);
        let shader = DiffuseShaderWithSpotlight::new(
            Float3::new(0.0, 0.0, 1.0),
            direction_to_light,
            ambient_factor,
            spotlight.clone(),
        );

        scene.models.push(Model::new(
            vertices,
            vertex_indices,
            texture_coords,
            texture_coord_indices,
            normals,
            normal_indices,
            transform,
            Box::new(shader),
        ));

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
        cam_transform.position.y = 2.0;
    }
}
