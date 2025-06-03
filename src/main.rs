use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

use rand::distr::{Distribution, Uniform};
use rastr::math::{Float2, Float3};
use rastr::render::RenderTarget;

fn create_test_image() -> RenderTarget {
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 748;

    const TRIANGLE_COUNT: usize = 250;

    let center = Float2::new(WIDTH as f32, HEIGHT as f32) / 2.0;

    let mut rng = rand::rng();
    let (low, high) = (Float2::zeros(), Float2::new(WIDTH as f32, HEIGHT as f32));
    let uniform_float2 = Uniform::new(low, high).unwrap();
    let uniform_color = Uniform::new(Float3::zeros(), Float3::ones()).unwrap();

    let points = (0..(3 * TRIANGLE_COUNT))
        .map(|_| center + (uniform_float2.sample(&mut rng) - center) * 0.3)
        .collect::<Vec<Float2>>();

    let colors = (0..TRIANGLE_COUNT)
        .map(|_| uniform_color.sample(&mut rng))
        .collect::<Vec<Float3>>();

    let mut target = RenderTarget::new(WIDTH, HEIGHT);

    target.clear(Float3::new(0.1, 0.1, 0.1));
    target.render(points, colors);

    target
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
        .buf
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
    let target = create_test_image();
    println!("done {:?}", start.elapsed());

    print!("Save image... ");
    let start = Instant::now();
    write_image_to_file(target, "art.ppm")?;
    println!("done {:?}", start.elapsed());

    Ok(())
}
