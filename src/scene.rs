use crate::model::{Model, read_obj_file};
use crate::math::Float3;
use crate::transform::Transform;

use rand::distr::{Distribution, Uniform};

pub struct Scene {
    pub models: Vec<Model>,
}

impl Scene {
    pub fn new() -> Self {
        let mut scene = Self { models: Vec::new() };

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
            Float3::new(0.0, 0.0, 3.0),
        );

        scene
            .models
            .push(Model::new(triangle_points, triangle_colors, transform));

        scene
    }
}
