use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

use rand::distr::{Distribution, Uniform};
use rastr::math::{Float2, Float3};

struct Image {
    width: usize,
    height: usize,
    buf: Vec<Float3>,
}

fn point_on_right_side_of_line(a: Float2, b: Float2, p: Float2) -> bool {
    let ap = p - a;
    let ab_perp = (b - a).perp();
    ap.dot(ab_perp) >= 0.0
}

fn point_in_triangle(a: Float2, b: Float2, c: Float2, p: Float2) -> bool {
    let side_ab = point_on_right_side_of_line(a, b, p);
    let side_bc = point_on_right_side_of_line(b, c, p);
    let side_ca = point_on_right_side_of_line(c, a, p);
    side_ab == side_bc && side_bc == side_ca
}

fn create_test_image() -> Image {
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 748;

    const TRIANGLE_COUNT: usize = 250;

    let center = Float2 {
        x: WIDTH as f32,
        y: HEIGHT as f32,
    } / 2.0;

    let mut rng = rand::rng();
    let (low, high) = (
        Float2 { x: 0.0, y: 0.0 },
        Float2 {
            x: WIDTH as f32,
            y: HEIGHT as f32,
        },
    );
    let uniform_float2 = Uniform::new(low, high).unwrap();
    let uniform_color = Uniform::new(
        Float3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Float3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    )
    .unwrap();

    let points = (0..(3 * TRIANGLE_COUNT))
        .map(|_| center + (uniform_float2.sample(&mut rng) - center) * 0.3)
        .collect::<Vec<Float2>>();

    // let velocities = (0..TRIANGLE_COUNT)
    //     .map(|_| (uniform_float2.sample(&mut rng) - center) * 0.5)
    //     .flat_map(|v| std::iter::repeat(v).take(3))
    //     .collect::<Vec<Float2>>();

    let triangle_colors = (0..TRIANGLE_COUNT)
        .map(|_| uniform_color.sample(&mut rng))
        .collect::<Vec<Float3>>();

    let mut buf: Vec<Float3> = Vec::new();
    buf.resize(
        WIDTH * HEIGHT,
        Float3 {
            x: 0.1,
            y: 0.1,
            z: 0.1,
        },
    );

    for (chunk, color) in points.chunks_exact(3).zip(triangle_colors.iter()) {
        // Determine chunk bounding box
        let (min_x, min_y, max_x, max_y) = (
            chunk[0].x.min(chunk[1].x).min(chunk[2].x),
            chunk[0].y.min(chunk[1].y).min(chunk[2].y),
            chunk[0].x.max(chunk[1].x).max(chunk[2].x),
            chunk[0].y.max(chunk[1].y).max(chunk[2].y),
        );

        let (bbox_start_x, bbox_start_y, bbox_end_x, bbox_end_y) = (
            min_x.floor().clamp(0.0, WIDTH as f32) as usize,
            min_y.floor().clamp(0.0, HEIGHT as f32) as usize,
            max_x.ceil().clamp(0.0, WIDTH as f32) as usize,
            max_y.ceil().clamp(0.0, HEIGHT as f32) as usize,
        );

        for y in bbox_start_y..bbox_end_y {
            for x in bbox_start_x..bbox_end_x {
                if point_in_triangle(
                    chunk[0],
                    chunk[1],
                    chunk[2],
                    Float2 {
                        x: x as f32,
                        y: y as f32,
                    },
                ) {
                    buf[y * WIDTH + x] = *color;
                }
            }
        }
    }

    // const A: Float2 = Float2 {
    //     x: 0.2 * (WIDTH as f32),
    //     y: 0.2 * (HEIGHT as f32),
    // };
    // const B: Float2 = Float2 {
    //     x: 0.7 * (WIDTH as f32),
    //     y: 0.4 * (HEIGHT as f32),
    // };
    // const C: Float2 = Float2 {
    //     x: 0.4 * (WIDTH as f32),
    //     y: 0.8 * (HEIGHT as f32),
    // };

    // let buf = (0..HEIGHT)
    //     .flat_map(|y| {
    //         (0..WIDTH).map(move |x| {
    //             let p = Float2 {
    //                 x: x as f32,
    //                 y: y as f32,
    //             };
    //             let inside = point_in_triangle(A, B, C, p);

    //             Float3 {
    //                 x: 0.0,
    //                 y: 0.0,
    //                 z: if inside { 1.0 } else { 0.0 },
    //             }
    //         })
    //     })
    //     .collect();

    Image {
        width: WIDTH,
        height: HEIGHT,
        buf,
    }
}

fn write_image_to_file(image: Image, path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    let header = [
        b"P3\n",
        image.width.to_string().as_bytes(),
        b" ",
        image.height.to_string().as_bytes(),
        b"\n255\n",
    ]
    .concat();
    file.write_all(&header)?;

    let content = image
        .buf
        .into_iter()
        .map(|px| {
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
        .flatten()
        .collect::<Vec<u8>>();

    file.write_all(&content)?;
    file.flush()?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    print!("Generate test image... ");
    let start = Instant::now();
    let image = create_test_image();
    println!("done {:?}", start.elapsed());

    print!("Save image... ");
    let start = Instant::now();
    write_image_to_file(image, "art.ppm")?;
    println!("done {:?}", start.elapsed());

    Ok(())
}
