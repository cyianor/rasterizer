use std::fs::File;
use std::io::prelude::*;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Float3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Add for Float3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul<f32> for Float3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Float3> for f32 {
    type Output = Float3;

    fn mul(self, rhs: Float3) -> Self::Output {
        rhs * self
    }
}

impl Neg for Float3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Float3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Float3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<&Float3> for Float3 {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        Float3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Float2 {
    x: f32,
    y: f32,
}

impl Add for Float2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<f32> for Float2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Float2> for f32 {
    type Output = Float2;

    fn mul(self, rhs: Float2) -> Self::Output {
        rhs * self
    }
}

impl Neg for Float2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Sub for Float2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Float2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<&Float2> for Float2 {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        Float2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Float2 {
    pub fn dot(self, other: Float2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn perp(self) -> Float2 {
        Float2 {
            x: self.y,
            y: -self.x,
        }
    }
}

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
    const WIDTH: usize = 256;
    const HEIGHT: usize = 256;

    const A: Float2 = Float2 {
        x: 0.2 * (WIDTH as f32),
        y: 0.2 * (HEIGHT as f32),
    };
    const B: Float2 = Float2 {
        x: 0.7 * (WIDTH as f32),
        y: 0.4 * (HEIGHT as f32),
    };
    const C: Float2 = Float2 {
        x: 0.4 * (WIDTH as f32),
        y: 0.8 * (HEIGHT as f32),
    };

    let buf = (0..HEIGHT)
        .map(|y| {
            (0..WIDTH).map(move |x| {
                let p = Float2 {
                    x: x as f32,
                    y: y as f32,
                };
                let inside = point_in_triangle(A, B, C, p);

                Float3 {
                    x: 0.0,
                    y: 0.0,
                    z: if inside { 1.0 } else { 0.0 },
                }
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

    Ok(())
}

fn main() -> std::io::Result<()> {
    let image = create_test_image();
    write_image_to_file(image, "art.ppm")
}
