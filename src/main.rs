use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

use rand::distr::{Distribution, Uniform};
use rastr::math::Float3;
use rastr::render::RenderTarget;
use rastr::model::{read_obj_file, Model};
use rastr::transform::Transform;

fn create_test_image() -> std::io::Result<RenderTarget> {
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 748;

    let triangle_points = read_obj_file("models/cube.obj")?;
    
    let mut rng = rand::rng();
    let uniform_color = Uniform::new(Float3::zeros(), Float3::ones()).unwrap();

    let triangle_colors = (0..triangle_points.len()/3)
        .map(|_| uniform_color.sample(&mut rng))
        .collect::<Vec<Float3>>();

    let transform = Transform::new(0.8, 0.45, Float3::new(0.0, 0.0, 5.0));

    let model = Model::new(triangle_points, triangle_colors, transform);

    let mut target = RenderTarget::new(WIDTH, HEIGHT, 60.0_f32.to_radians());

    target.clear(Float3::new(0.1, 0.1, 0.1));
    target.render(model);

    Ok(target)
}

fn write_image_to_file(target: RenderTarget, path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    let header = [
        b"P3\n",
        target.width.to_string().as_bytes(),
        b" ",
        target.height.to_string().as_bytes(),
        b"\n255\n",
    ]
    .concat();
    file.write_all(&header)?;

    let content: Vec<u8> = target
        .color_buffer
        .into_iter()
        .flat_map(|px| {
            [
                ((px.x * 255.0_f32).floor() as i32).to_string().as_bytes(),
                b" ",
                ((px.y * 255.0_f32).floor() as i32).to_string().as_bytes(),
                b" ",
                ((px.z * 255.0_f32).floor() as i32).to_string().as_bytes(),
                b"\n",
            ]
            .concat()
        })
        .collect();

    file.write_all(&content)?;
    file.flush()?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    print!("Generate test image... ");
    let start = Instant::now();
    let target = create_test_image()?;
    println!("done {:?}", start.elapsed());

    print!("Save image... ");
    let start = Instant::now();
    write_image_to_file(target, "art.ppm")?;
    println!("done {:?}", start.elapsed());

    Ok(())
}
