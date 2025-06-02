use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Float3 {
    x: f32,
    y: f32,
    z: f32,
}

struct Image {
    width: usize,
    height: usize,
    buf: Vec<Float3>,
}


fn create_test_image() -> Image {
    const WIDTH: usize = 320;
    const HEIGHT: usize = 240;
    
    let buf = (0..HEIGHT)
        .map(|y| {
            (0..WIDTH).map(move |x| Float3 {
                x: x as f32 / ((WIDTH - 1) as f32),
                y: y as f32 / ((HEIGHT - 1) as f32),
                z: 0.0,
            })
        })
        .flatten()
        .collect();

    Image {
        width: WIDTH,
        height: HEIGHT,
        buf,
    }
}

fn write_image_to_file(image: Image) -> std::io::Result<()> {
    let mut file = File::create("art.ppm")?;

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

    Ok(())
}

fn main() -> std::io::Result<()> {
    let image = create_test_image();
    write_image_to_file(image)
}
