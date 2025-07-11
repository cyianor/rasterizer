use crate::camera::Camera;
use crate::light::SpotLight;
use crate::math::Float3;
use crate::model::{Model, read_obj_file};
use crate::render::RenderTarget;
use crate::shader::DiffuseShaderWithSpotlight;
use crate::transform::Transform;
use raylib::RaylibHandle;
use raylib::ffi::KeyboardKey;
use std::cell::RefCell;
use std::rc::Rc;

/// Description of the rendered scene
pub struct Scene {
    /// Main camera
    pub camera: Camera,
    /// Triangle models
    pub models: Vec<Model>,
    /// Spotlights
    pub spotlights: Vec<Rc<RefCell<SpotLight>>>,
    total_frame_time: f32,
    /// Average time necessary to compute a frame within the last second
    pub average_frame_time: f32,
    frame_counter: i32,
    /// Number of frames computed within the last second
    pub last_frame_counter: i32,
}

impl Scene {
    /// Create a new scene
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
            spotlights: Vec::new(),
            total_frame_time: 0.0,
            average_frame_time: 0.0,
            frame_counter: 0,
            last_frame_counter: 0,
        };

        let ambient_factor = 0.5f32;
        let direction_to_light = Float3::new(1.0, 1.0, 0.0).normalized();
        let spotlight = Rc::new(RefCell::new(SpotLight::new(
            Float3::new(1.0, 1.0, 1.0),
            Float3::new(-8.0, 8.0, 0.0),
            Float3::new(0.0, 0.0, 0.0),
            30f32.to_radians(),
            256,
            256,
        )));

        let (
            vertices,
            vertex_indices,
            texture_coords,
            texture_coord_indices,
            normals,
            normal_indices,
        ) = read_obj_file("models/dragon.obj").unwrap();

        // Generate smooth normals which are an average of all normals at a specific vertex
        let mut visited = Vec::new();
        let mut smooth_normals = Vec::new();
        let mut smooth_normal_indices = normal_indices.clone();
        let mut i = 0;
        for vidx0 in vertex_indices.iter() {
            if visited.contains(vidx0) {
                continue;
            }

            let mut count = 0;
            let mut vertex_normal = Float3::zeros();

            for (j, (vidx1, nidx)) in vertex_indices.iter().zip(normal_indices.iter()).enumerate() {
                if *vidx1 == *vidx0 {
                    vertex_normal += normals[*nidx];
                    count += 1;
                    smooth_normal_indices[j] = i;
                }
            }
            
            smooth_normals.push(vertex_normal / (count as f32));
            visited.push(*vidx0);
            i += 1;
        }

        let transform = Transform::new(0.0, 0.0, 0.0, Float3::new(0.0, 4.0, 0.0), Float3::ones());

        let shader = DiffuseShaderWithSpotlight::new(
            Float3::new(0.0, 1.0, 0.0),
            direction_to_light,
            ambient_factor,
            Rc::clone(&spotlight),
        );

        scene.models.push(Model::new(
            vertices,
            vertex_indices,
            texture_coords,
            texture_coord_indices,
            // normals,
            // normal_indices,
            smooth_normals,
            smooth_normal_indices,
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
        ) = read_obj_file("models/cube.obj").unwrap();

        let transform = Transform::new(0.0, 0.0, 0.0, Float3::new(-3.0, 1.0, 0.0), Float3::ones());

        let shader = DiffuseShaderWithSpotlight::new(
            Float3::new(1.0, 0.0, 0.0),
            direction_to_light,
            ambient_factor,
            Rc::clone(&spotlight),
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
        ) = read_obj_file("models/cube.obj").unwrap();

        let transform = Transform::new(
            0.0,
            0.0,
            0.0,
            Float3::new(-8.0, 8.0, 0.0),
            0.1 * Float3::ones(),
        );

        let shader = DiffuseShaderWithSpotlight::new(
            Float3::new(1.0, 1.0, 1.0),
            direction_to_light,
            ambient_factor,
            Rc::clone(&spotlight),
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
        ) = read_obj_file("models/floor.obj").unwrap();

        let transform = Transform::new(0.0, 0.0, 0.0, Float3::new(0.0, 0.0, 0.0), Float3::ones());

        // let texture = Texture::from_png("models/checker-map_tho.png");
        // let shader = TextureShader::new(texture);
        let shader = DiffuseShaderWithSpotlight::new(
            Float3::new(0.0, 0.0, 1.0),
            direction_to_light,
            ambient_factor,
            Rc::clone(&spotlight),
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
        scene.spotlights.push(spotlight);

        scene
    }

    /// Update the scene
    /// 
    /// In this function animations, model and camera movement are handled.
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
        self.models[1].transform.yaw += delta_time;

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
