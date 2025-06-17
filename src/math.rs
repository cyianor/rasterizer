use rand::{
    Rng,
    distr::uniform::{SampleUniform, UniformFloat, UniformSampler},
};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

impl Float2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zeros() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn ones() -> Self {
        Self::new(1.0, 1.0)
    }

    pub fn unit_x() -> Self {
        Self::new(1.0, 0.0)
    }

    pub fn unit_y() -> Self {
        Self::new(0.0, 1.0)
    }

    pub fn dot(&self, other: Float2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn perp(self) -> Float2 {
        Float2 {
            x: self.y,
            y: -self.x,
        }
    }

    pub fn lerp(&self, other: Float2, t: f32) -> Self {
        self + t * (other - self)
    }
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

impl Add<&Float2> for Float2 {
    type Output = Self;

    fn add(self, rhs: &Float2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Float2> for &Float2 {
    type Output = Float2;

    fn add(self, rhs: Float2) -> Self::Output {
        Float2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<'a, 'b> Add<&'a Float2> for &'b Float2 {
    type Output = Float2;

    fn add(self, rhs: &'a Float2) -> Self::Output {
        Float2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<f32> for Float2 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Add<&f32> for Float2 {
    type Output = Self;

    fn add(self, rhs: &f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Add<f32> for &Float2 {
    type Output = Float2;

    fn add(self, rhs: f32) -> Self::Output {
        Float2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl<'a, 'b> Add<&'a f32> for &'b Float2 {
    type Output = Float2;

    fn add(self, rhs: &'a f32) -> Self::Output {
        Float2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Add<Float2> for f32 {
    type Output = Float2;

    fn add(self, rhs: Float2) -> Self::Output {
        Float2 {
            x: self + rhs.x,
            y: self + rhs.y,
        }
    }
}

impl Add<&Float2> for f32 {
    type Output = Float2;

    fn add(self, rhs: &Float2) -> Self::Output {
        Float2 {
            x: self + rhs.x,
            y: self + rhs.y,
        }
    }
}

impl Add<Float2> for &f32 {
    type Output = Float2;

    fn add(self, rhs: Float2) -> Self::Output {
        Float2 {
            x: self + rhs.x,
            y: self + rhs.y,
        }
    }
}

impl<'a, 'b> Add<&'a Float2> for &'b f32 {
    type Output = Float2;

    fn add(self, rhs: &'a Float2) -> Self::Output {
        Float2 {
            x: self + rhs.x,
            y: self + rhs.y,
        }
    }
}

impl AddAssign for Float2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<&Float2> for Float2 {
    fn add_assign(&mut self, rhs: &Float2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<f32> for Float2 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl AddAssign<&f32> for Float2 {
    fn add_assign(&mut self, rhs: &f32) {
        self.x += rhs;
        self.y += rhs;
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

impl Sub<Float2> for &Float2 {
    type Output = Float2;

    fn sub(self, rhs: Float2) -> Self::Output {
        Float2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<&Float2> for Float2 {
    type Output = Self;

    fn sub(self, rhs: &Float2) -> Self::Output {
        Float2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<'a, 'b> Sub<&'b Float2> for &'a Float2 {
    type Output = Float2;

    fn sub(self, rhs: &'b Float2) -> Self::Output {
        Float2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<f32> for Float2 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Float2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl Sub<f32> for &Float2 {
    type Output = Float2;

    fn sub(self, rhs: f32) -> Self::Output {
        Float2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl Sub<&f32> for Float2 {
    type Output = Self;

    fn sub(self, rhs: &f32) -> Self::Output {
        Float2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl<'a, 'b> Sub<&'b f32> for &'a Float2 {
    type Output = Float2;

    fn sub(self, rhs: &'b f32) -> Self::Output {
        Float2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl Sub<Float2> for f32 {
    type Output = Float2;

    fn sub(self, rhs: Float2) -> Self::Output {
        Float2 {
            x: self - rhs.x,
            y: self - rhs.y,
        }
    }
}

impl Sub<Float2> for &f32 {
    type Output = Float2;

    fn sub(self, rhs: Float2) -> Self::Output {
        Float2 {
            x: self - rhs.x,
            y: self - rhs.y,
        }
    }
}

impl Sub<&Float2> for f32 {
    type Output = Float2;

    fn sub(self, rhs: &Float2) -> Self::Output {
        Float2 {
            x: self - rhs.x,
            y: self - rhs.y,
        }
    }
}

impl<'a, 'b> Sub<&'b Float2> for &'a f32 {
    type Output = Float2;

    fn sub(self, rhs: &'b Float2) -> Self::Output {
        Float2 {
            x: self - rhs.x,
            y: self - rhs.y,
        }
    }
}

impl SubAssign for Float2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl SubAssign<&Float2> for Float2 {
    fn sub_assign(&mut self, rhs: &Float2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl SubAssign<f32> for Float2 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl SubAssign<&f32> for Float2 {
    fn sub_assign(&mut self, rhs: &f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl Mul for Float2 {
    type Output = Self;

    fn mul(self, rhs: Float2) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<Float2> for &Float2 {
    type Output = Float2;

    fn mul(self, rhs: Float2) -> Self::Output {
        Float2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<&Float2> for Float2 {
    type Output = Self;

    fn mul(self, rhs: &Float2) -> Self::Output {
        Float2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<'a, 'b> Mul<&'a Float2> for &'b Float2 {
    type Output = Float2;

    fn mul(self, rhs: &'a Float2) -> Self::Output {
        Float2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
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

impl Mul<f32> for &Float2 {
    type Output = Float2;

    fn mul(self, rhs: f32) -> Self::Output {
        Float2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<&f32> for Float2 {
    type Output = Self;

    fn mul(self, rhs: &f32) -> Self::Output {
        Float2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<'a, 'b> Mul<&'a f32> for &'b Float2 {
    type Output = Float2;

    fn mul(self, rhs: &'a f32) -> Self::Output {
        Float2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Float2> for f32 {
    type Output = Float2;

    fn mul(self, rhs: Float2) -> Self::Output {
        Float2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Mul<Float2> for &f32 {
    type Output = Float2;

    fn mul(self, rhs: Float2) -> Self::Output {
        Float2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Mul<&Float2> for f32 {
    type Output = Float2;

    fn mul(self, rhs: &Float2) -> Self::Output {
        Float2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl<'a, 'b> Mul<&'a Float2> for &'b f32 {
    type Output = Float2;

    fn mul(self, rhs: &'a Float2) -> Self::Output {
        Float2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl MulAssign for Float2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl MulAssign<&Float2> for Float2 {
    fn mul_assign(&mut self, rhs: &Float2) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl MulAssign<f32> for Float2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl MulAssign<&f32> for Float2 {
    fn mul_assign(&mut self, rhs: &f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div for Float2 {
    type Output = Self;

    fn div(self, rhs: Float2) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<Float2> for &Float2 {
    type Output = Float2;

    fn div(self, rhs: Float2) -> Self::Output {
        Float2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<&Float2> for Float2 {
    type Output = Self;

    fn div(self, rhs: &Float2) -> Self::Output {
        Float2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<'a, 'b> Div<&'a Float2> for &'b Float2 {
    type Output = Float2;

    fn div(self, rhs: &'a Float2) -> Self::Output {
        Float2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<f32> for Float2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Float2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Div<f32> for &Float2 {
    type Output = Float2;

    fn div(self, rhs: f32) -> Self::Output {
        Float2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Div<&f32> for Float2 {
    type Output = Self;

    fn div(self, rhs: &f32) -> Self::Output {
        Float2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<'a, 'b> Div<&'a f32> for &'b Float2 {
    type Output = Float2;

    fn div(self, rhs: &'a f32) -> Self::Output {
        Float2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Div<Float2> for f32 {
    type Output = Float2;

    fn div(self, rhs: Float2) -> Self::Output {
        Float2 {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
}

impl Div<Float2> for &f32 {
    type Output = Float2;

    fn div(self, rhs: Float2) -> Self::Output {
        Float2 {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
}

impl Div<&Float2> for f32 {
    type Output = Float2;

    fn div(self, rhs: &Float2) -> Self::Output {
        Float2 {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
}

impl<'a, 'b> Div<&'a Float2> for &'b f32 {
    type Output = Float2;

    fn div(self, rhs: &'a Float2) -> Self::Output {
        Float2 {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
}

impl DivAssign for Float2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl DivAssign<&Float2> for Float2 {
    fn div_assign(&mut self, rhs: &Float2) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl DivAssign<f32> for Float2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl DivAssign<&f32> for Float2 {
    fn div_assign(&mut self, rhs: &f32) {
        self.x /= rhs;
        self.y /= rhs;
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

impl Neg for &Float2 {
    type Output = Float2;

    fn neg(self) -> Self::Output {
        Float2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UniformFloat2 {
    x: UniformFloat<f32>,
    y: UniformFloat<f32>,
}

impl UniformSampler for UniformFloat2 {
    type X = Float2;

    fn new<B1, B2>(low: B1, high: B2) -> Result<Self, rand::distr::uniform::Error>
    where
        B1: rand::distr::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distr::uniform::SampleBorrow<Self::X> + Sized,
    {
        Ok(UniformFloat2 {
            x: UniformFloat::<f32>::new(low.borrow().x, high.borrow().x)?,
            y: UniformFloat::<f32>::new(low.borrow().y, high.borrow().y)?,
        })
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Result<Self, rand::distr::uniform::Error>
    where
        B1: rand::distr::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distr::uniform::SampleBorrow<Self::X> + Sized,
    {
        Ok(UniformFloat2 {
            x: UniformFloat::<f32>::new_inclusive(low.borrow().x, high.borrow().x)?,
            y: UniformFloat::<f32>::new_inclusive(low.borrow().y, high.borrow().y)?,
        })
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        Float2 {
            x: self.x.sample(rng),
            y: self.y.sample(rng),
        }
    }
}

impl SampleUniform for Float2 {
    type Sampler = UniformFloat2;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Float3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zeros() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn ones() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn unit_x() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn unit_y() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn unit_z() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn dot(&self, other: Float3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Float3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let norm = self.norm();
        if norm < 1e-8 {
            Self::zeros()
        } else {
            self / self.norm()
        }
    }

    pub fn lerp(&self, other: Float3, t: f32) -> Self {
        self + t * (other - self)
    }
}

impl Add for Float3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<&Float3> for Float3 {
    type Output = Self;

    fn add(self, rhs: &Float3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Float3> for &Float3 {
    type Output = Float3;

    fn add(self, rhs: Float3) -> Self::Output {
        Float3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<'a, 'b> Add<&'a Float3> for &'b Float3 {
    type Output = Float3;

    fn add(self, rhs: &'a Float3) -> Self::Output {
        Float3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<f32> for Float3 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Add<&f32> for Float3 {
    type Output = Self;

    fn add(self, rhs: &f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Add<f32> for &Float3 {
    type Output = Float3;

    fn add(self, rhs: f32) -> Self::Output {
        Float3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl<'a, 'b> Add<&'a f32> for &'b Float3 {
    type Output = Float3;

    fn add(self, rhs: &'a f32) -> Self::Output {
        Float3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Add<Float3> for f32 {
    type Output = Float3;

    fn add(self, rhs: Float3) -> Self::Output {
        Float3 {
            x: self + rhs.x,
            y: self + rhs.y,
            z: self + rhs.z,
        }
    }
}

impl Add<&Float3> for f32 {
    type Output = Float3;

    fn add(self, rhs: &Float3) -> Self::Output {
        Float3 {
            x: self + rhs.x,
            y: self + rhs.y,
            z: self + rhs.z,
        }
    }
}

impl Add<Float3> for &f32 {
    type Output = Float3;

    fn add(self, rhs: Float3) -> Self::Output {
        Float3 {
            x: self + rhs.x,
            y: self + rhs.y,
            z: self + rhs.z,
        }
    }
}

impl<'a, 'b> Add<&'a Float3> for &'b f32 {
    type Output = Float3;

    fn add(self, rhs: &'a Float3) -> Self::Output {
        Float3 {
            x: self + rhs.x,
            y: self + rhs.y,
            z: self + rhs.z,
        }
    }
}

impl AddAssign for Float3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<&Float3> for Float3 {
    fn add_assign(&mut self, rhs: &Float3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<f32> for Float3 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl AddAssign<&f32> for Float3 {
    fn add_assign(&mut self, rhs: &f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
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

impl Sub<Float3> for &Float3 {
    type Output = Float3;

    fn sub(self, rhs: Float3) -> Self::Output {
        Float3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<&Float3> for Float3 {
    type Output = Self;

    fn sub(self, rhs: &Float3) -> Self::Output {
        Float3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<'a, 'b> Sub<&'b Float3> for &'a Float3 {
    type Output = Float3;

    fn sub(self, rhs: &'b Float3) -> Self::Output {
        Float3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<f32> for Float3 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Float3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl Sub<f32> for &Float3 {
    type Output = Float3;

    fn sub(self, rhs: f32) -> Self::Output {
        Float3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl Sub<&f32> for Float3 {
    type Output = Self;

    fn sub(self, rhs: &f32) -> Self::Output {
        Float3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl<'a, 'b> Sub<&'b f32> for &'a Float3 {
    type Output = Float3;

    fn sub(self, rhs: &'b f32) -> Self::Output {
        Float3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl Sub<Float3> for f32 {
    type Output = Float3;

    fn sub(self, rhs: Float3) -> Self::Output {
        Float3 {
            x: self - rhs.x,
            y: self - rhs.y,
            z: self - rhs.z,
        }
    }
}

impl Sub<Float3> for &f32 {
    type Output = Float3;

    fn sub(self, rhs: Float3) -> Self::Output {
        Float3 {
            x: self - rhs.x,
            y: self - rhs.y,
            z: self - rhs.z,
        }
    }
}

impl Sub<&Float3> for f32 {
    type Output = Float3;

    fn sub(self, rhs: &Float3) -> Self::Output {
        Float3 {
            x: self - rhs.x,
            y: self - rhs.y,
            z: self - rhs.z,
        }
    }
}

impl<'a, 'b> Sub<&'b Float3> for &'a f32 {
    type Output = Float3;

    fn sub(self, rhs: &'b Float3) -> Self::Output {
        Float3 {
            x: self - rhs.x,
            y: self - rhs.y,
            z: self - rhs.z,
        }
    }
}

impl SubAssign for Float3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl SubAssign<&Float3> for Float3 {
    fn sub_assign(&mut self, rhs: &Float3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl SubAssign<f32> for Float3 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl SubAssign<&f32> for Float3 {
    fn sub_assign(&mut self, rhs: &f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl Mul for Float3 {
    type Output = Self;

    fn mul(self, rhs: Float3) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Float3> for &Float3 {
    type Output = Float3;

    fn mul(self, rhs: Float3) -> Self::Output {
        Float3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<&Float3> for Float3 {
    type Output = Self;

    fn mul(self, rhs: &Float3) -> Self::Output {
        Float3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<'a, 'b> Mul<&'a Float3> for &'b Float3 {
    type Output = Float3;

    fn mul(self, rhs: &'a Float3) -> Self::Output {
        Float3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
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

impl Mul<f32> for &Float3 {
    type Output = Float3;

    fn mul(self, rhs: f32) -> Self::Output {
        Float3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<&f32> for Float3 {
    type Output = Self;

    fn mul(self, rhs: &f32) -> Self::Output {
        Float3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<'a, 'b> Mul<&'a f32> for &'b Float3 {
    type Output = Float3;

    fn mul(self, rhs: &'a f32) -> Self::Output {
        Float3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Float3> for f32 {
    type Output = Float3;

    fn mul(self, rhs: Float3) -> Self::Output {
        Float3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<Float3> for &f32 {
    type Output = Float3;

    fn mul(self, rhs: Float3) -> Self::Output {
        Float3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<&Float3> for f32 {
    type Output = Float3;

    fn mul(self, rhs: &Float3) -> Self::Output {
        Float3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl<'a, 'b> Mul<&'a Float3> for &'b f32 {
    type Output = Float3;

    fn mul(self, rhs: &'a Float3) -> Self::Output {
        Float3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl MulAssign for Float3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl MulAssign<&Float3> for Float3 {
    fn mul_assign(&mut self, rhs: &Float3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl MulAssign<f32> for Float3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl MulAssign<&f32> for Float3 {
    fn mul_assign(&mut self, rhs: &f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div for Float3 {
    type Output = Self;

    fn div(self, rhs: Float3) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<Float3> for &Float3 {
    type Output = Float3;

    fn div(self, rhs: Float3) -> Self::Output {
        Float3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<&Float3> for Float3 {
    type Output = Self;

    fn div(self, rhs: &Float3) -> Self::Output {
        Float3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<'a, 'b> Div<&'a Float3> for &'b Float3 {
    type Output = Float3;

    fn div(self, rhs: &'a Float3) -> Self::Output {
        Float3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<f32> for Float3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Float3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<f32> for &Float3 {
    type Output = Float3;

    fn div(self, rhs: f32) -> Self::Output {
        Float3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<&f32> for Float3 {
    type Output = Self;

    fn div(self, rhs: &f32) -> Self::Output {
        Float3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<'a, 'b> Div<&'a f32> for &'b Float3 {
    type Output = Float3;

    fn div(self, rhs: &'a f32) -> Self::Output {
        Float3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<Float3> for f32 {
    type Output = Float3;

    fn div(self, rhs: Float3) -> Self::Output {
        Float3 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}

impl Div<Float3> for &f32 {
    type Output = Float3;

    fn div(self, rhs: Float3) -> Self::Output {
        Float3 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}

impl Div<&Float3> for f32 {
    type Output = Float3;

    fn div(self, rhs: &Float3) -> Self::Output {
        Float3 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}

impl<'a, 'b> Div<&'a Float3> for &'b f32 {
    type Output = Float3;

    fn div(self, rhs: &'a Float3) -> Self::Output {
        Float3 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}

impl DivAssign for Float3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl DivAssign<&Float3> for Float3 {
    fn div_assign(&mut self, rhs: &Float3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl DivAssign<f32> for Float3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl DivAssign<&f32> for Float3 {
    fn div_assign(&mut self, rhs: &f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
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

impl Neg for &Float3 {
    type Output = Float3;

    fn neg(self) -> Self::Output {
        Float3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UniformFloat3 {
    x: UniformFloat<f32>,
    y: UniformFloat<f32>,
    z: UniformFloat<f32>,
}

impl UniformSampler for UniformFloat3 {
    type X = Float3;

    fn new<B1, B2>(low: B1, high: B2) -> Result<Self, rand::distr::uniform::Error>
    where
        B1: rand::distr::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distr::uniform::SampleBorrow<Self::X> + Sized,
    {
        Ok(UniformFloat3 {
            x: UniformFloat::<f32>::new(low.borrow().x, high.borrow().x)?,
            y: UniformFloat::<f32>::new(low.borrow().y, high.borrow().y)?,
            z: UniformFloat::<f32>::new(low.borrow().z, high.borrow().z)?,
        })
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Result<Self, rand::distr::uniform::Error>
    where
        B1: rand::distr::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distr::uniform::SampleBorrow<Self::X> + Sized,
    {
        Ok(UniformFloat3 {
            x: UniformFloat::<f32>::new_inclusive(low.borrow().x, high.borrow().x)?,
            y: UniformFloat::<f32>::new_inclusive(low.borrow().y, high.borrow().y)?,
            z: UniformFloat::<f32>::new_inclusive(low.borrow().z, high.borrow().z)?,
        })
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        Float3 {
            x: self.x.sample(rng),
            y: self.y.sample(rng),
            z: self.z.sample(rng),
        }
    }
}

impl SampleUniform for Float3 {
    type Sampler = UniformFloat3;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Float4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Float4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn from_vector(v: Float3) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w: 0.0,
        }
    }

    pub fn from_point(p: Float3) -> Self {
        Self {
            x: p.x,
            y: p.y,
            z: p.z,
            w: 1.0,
        }
    }

    pub fn zeros() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn ones() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        }
    }

    pub fn unit_x() -> Self {
        Self::new(1.0, 0.0, 0.0, 0.0)
    }

    pub fn unit_y() -> Self {
        Self::new(0.0, 1.0, 0.0, 0.0)
    }

    pub fn unit_z() -> Self {
        Self::new(0.0, 0.0, 1.0, 0.0)
    }

    pub fn unit_w() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }

    pub fn xyz(&self) -> Float3 {
        Float3::new(self.x, self.y, self.z)
    }

    pub fn dot(&self, other: &Float4) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let norm = self.norm();
        if norm < 1e-8 {
            Self::zeros()
        } else {
            self / self.norm()
        }
    }

    pub fn lerp(&self, other: &Float4, t: f32) -> Self {
        self + t * (other - self)
    }
}

impl Add for Float4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Add<&Float4> for Float4 {
    type Output = Self;

    fn add(self, rhs: &Float4) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Add<Float4> for &Float4 {
    type Output = Float4;

    fn add(self, rhs: Float4) -> Self::Output {
        Float4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl<'a, 'b> Add<&'a Float4> for &'b Float4 {
    type Output = Float4;

    fn add(self, rhs: &'a Float4) -> Self::Output {
        Float4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Add<f32> for Float4 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
            w: self.w + rhs,
        }
    }
}

impl Add<&f32> for Float4 {
    type Output = Self;

    fn add(self, rhs: &f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
            w: self.w + rhs,
        }
    }
}

impl Add<f32> for &Float4 {
    type Output = Float4;

    fn add(self, rhs: f32) -> Self::Output {
        Float4 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
            w: self.w + rhs,
        }
    }
}

impl<'a, 'b> Add<&'a f32> for &'b Float4 {
    type Output = Float4;

    fn add(self, rhs: &'a f32) -> Self::Output {
        Float4 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
            w: self.w + rhs,
        }
    }
}

impl Add<Float4> for f32 {
    type Output = Float4;

    fn add(self, rhs: Float4) -> Self::Output {
        Float4 {
            x: self + rhs.x,
            y: self + rhs.y,
            z: self + rhs.z,
            w: self + rhs.w,
        }
    }
}

impl Add<&Float4> for f32 {
    type Output = Float4;

    fn add(self, rhs: &Float4) -> Self::Output {
        Float4 {
            x: self + rhs.x,
            y: self + rhs.y,
            z: self + rhs.z,
            w: self + rhs.w,
        }
    }
}

impl Add<Float4> for &f32 {
    type Output = Float4;

    fn add(self, rhs: Float4) -> Self::Output {
        Float4 {
            x: self + rhs.x,
            y: self + rhs.y,
            z: self + rhs.z,
            w: self + rhs.w,
        }
    }
}

impl<'a, 'b> Add<&'a Float4> for &'b f32 {
    type Output = Float4;

    fn add(self, rhs: &'a Float4) -> Self::Output {
        Float4 {
            x: self + rhs.x,
            y: self + rhs.y,
            z: self + rhs.z,
            w: self + rhs.w,
        }
    }
}

impl AddAssign for Float4 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl AddAssign<&Float4> for Float4 {
    fn add_assign(&mut self, rhs: &Float4) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl AddAssign<f32> for Float4 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
        self.w += rhs;
    }
}

impl AddAssign<&f32> for Float4 {
    fn add_assign(&mut self, rhs: &f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
        self.w += rhs;
    }
}

impl Sub for Float4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Float4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Sub<Float4> for &Float4 {
    type Output = Float4;

    fn sub(self, rhs: Float4) -> Self::Output {
        Float4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Sub<&Float4> for Float4 {
    type Output = Self;

    fn sub(self, rhs: &Float4) -> Self::Output {
        Float4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl<'a, 'b> Sub<&'b Float4> for &'a Float4 {
    type Output = Float4;

    fn sub(self, rhs: &'b Float4) -> Self::Output {
        Float4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Sub<f32> for Float4 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Float4 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
            w: self.w - rhs,
        }
    }
}

impl Sub<f32> for &Float4 {
    type Output = Float4;

    fn sub(self, rhs: f32) -> Self::Output {
        Float4 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
            w: self.w - rhs,
        }
    }
}

impl Sub<&f32> for Float4 {
    type Output = Self;

    fn sub(self, rhs: &f32) -> Self::Output {
        Float4 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
            w: self.w - rhs,
        }
    }
}

impl<'a, 'b> Sub<&'b f32> for &'a Float4 {
    type Output = Float4;

    fn sub(self, rhs: &'b f32) -> Self::Output {
        Float4 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
            w: self.w - rhs,
        }
    }
}

impl Sub<Float4> for f32 {
    type Output = Float4;

    fn sub(self, rhs: Float4) -> Self::Output {
        Float4 {
            x: self - rhs.x,
            y: self - rhs.y,
            z: self - rhs.z,
            w: self - rhs.w,
        }
    }
}

impl Sub<Float4> for &f32 {
    type Output = Float4;

    fn sub(self, rhs: Float4) -> Self::Output {
        Float4 {
            x: self - rhs.x,
            y: self - rhs.y,
            z: self - rhs.z,
            w: self - rhs.w,
        }
    }
}

impl Sub<&Float4> for f32 {
    type Output = Float4;

    fn sub(self, rhs: &Float4) -> Self::Output {
        Float4 {
            x: self - rhs.x,
            y: self - rhs.y,
            z: self - rhs.z,
            w: self - rhs.w,
        }
    }
}

impl<'a, 'b> Sub<&'b Float4> for &'a f32 {
    type Output = Float4;

    fn sub(self, rhs: &'b Float4) -> Self::Output {
        Float4 {
            x: self - rhs.x,
            y: self - rhs.y,
            z: self - rhs.z,
            w: self - rhs.w,
        }
    }
}

impl SubAssign for Float4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl SubAssign<&Float4> for Float4 {
    fn sub_assign(&mut self, rhs: &Float4) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl SubAssign<f32> for Float4 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
        self.w -= rhs;
    }
}

impl SubAssign<&f32> for Float4 {
    fn sub_assign(&mut self, rhs: &f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
        self.w -= rhs;
    }
}

impl Mul for Float4 {
    type Output = Self;

    fn mul(self, rhs: Float4) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl Mul<Float4> for &Float4 {
    type Output = Float4;

    fn mul(self, rhs: Float4) -> Self::Output {
        Float4 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl Mul<&Float4> for Float4 {
    type Output = Self;

    fn mul(self, rhs: &Float4) -> Self::Output {
        Float4 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl<'a, 'b> Mul<&'a Float4> for &'b Float4 {
    type Output = Float4;

    fn mul(self, rhs: &'a Float4) -> Self::Output {
        Float4 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl Mul<f32> for Float4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Mul<f32> for &Float4 {
    type Output = Float4;

    fn mul(self, rhs: f32) -> Self::Output {
        Float4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Mul<&f32> for Float4 {
    type Output = Self;

    fn mul(self, rhs: &f32) -> Self::Output {
        Float4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl<'a, 'b> Mul<&'a f32> for &'b Float4 {
    type Output = Float4;

    fn mul(self, rhs: &'a f32) -> Self::Output {
        Float4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Mul<Float4> for f32 {
    type Output = Float4;

    fn mul(self, rhs: Float4) -> Self::Output {
        Float4 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w,
        }
    }
}

impl Mul<Float4> for &f32 {
    type Output = Float4;

    fn mul(self, rhs: Float4) -> Self::Output {
        Float4 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w,
        }
    }
}

impl Mul<&Float4> for f32 {
    type Output = Float4;

    fn mul(self, rhs: &Float4) -> Self::Output {
        Float4 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w,
        }
    }
}

impl<'a, 'b> Mul<&'a Float4> for &'b f32 {
    type Output = Float4;

    fn mul(self, rhs: &'a Float4) -> Self::Output {
        Float4 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w,
        }
    }
}

impl MulAssign for Float4 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}

impl MulAssign<&Float4> for Float4 {
    fn mul_assign(&mut self, rhs: &Float4) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}

impl MulAssign<f32> for Float4 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl MulAssign<&f32> for Float4 {
    fn mul_assign(&mut self, rhs: &f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl Div for Float4 {
    type Output = Self;

    fn div(self, rhs: Float4) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}

impl Div<Float4> for &Float4 {
    type Output = Float4;

    fn div(self, rhs: Float4) -> Self::Output {
        Float4 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}

impl Div<&Float4> for Float4 {
    type Output = Self;

    fn div(self, rhs: &Float4) -> Self::Output {
        Float4 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}

impl<'a, 'b> Div<&'a Float4> for &'b Float4 {
    type Output = Float4;

    fn div(self, rhs: &'a Float4) -> Self::Output {
        Float4 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}

impl Div<f32> for Float4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Float4 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl Div<f32> for &Float4 {
    type Output = Float4;

    fn div(self, rhs: f32) -> Self::Output {
        Float4 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl Div<&f32> for Float4 {
    type Output = Self;

    fn div(self, rhs: &f32) -> Self::Output {
        Float4 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl<'a, 'b> Div<&'a f32> for &'b Float4 {
    type Output = Float4;

    fn div(self, rhs: &'a f32) -> Self::Output {
        Float4 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl DivAssign for Float4 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}

impl DivAssign<&Float4> for Float4 {
    fn div_assign(&mut self, rhs: &Float4) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}

impl DivAssign<f32> for Float4 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}

impl DivAssign<&f32> for Float4 {
    fn div_assign(&mut self, rhs: &f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}

impl Neg for Float4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Neg for &Float4 {
    type Output = Float4;

    fn neg(self) -> Self::Output {
        Float4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

#[derive(Debug)]
pub struct Float4x4 {
    pub r1: Float4,
    pub r2: Float4,
    pub r3: Float4,
    pub r4: Float4,
}

impl Float4x4 {
    pub fn new(r1: Float4, r2: Float4, r3: Float4, r4: Float4) -> Self {
        Self { r1, r2, r3, r4 }
    }

    pub fn from_columns(c1: Float4, c2: Float4, c3: Float4, c4: Float4) -> Self {
        Self {
            r1: Float4::new(c1.x, c2.x, c3.x, c4.x),
            r2: Float4::new(c1.y, c2.y, c3.y, c4.y),
            r3: Float4::new(c1.z, c2.z, c3.z, c4.z),
            r4: Float4::new(c1.w, c2.w, c3.w, c4.w),
        }
    }

    pub fn zeros() -> Self {
        Self::new(
            Float4::zeros(),
            Float4::zeros(),
            Float4::zeros(),
            Float4::zeros(),
        )
    }

    pub fn eye() -> Self {
        Self::new(
            Float4::unit_x(),
            Float4::unit_y(),
            Float4::unit_z(),
            Float4::unit_w(),
        )
    }

    pub fn translation(translation: Float3) -> Self {
        Self::new(
            Float4::unit_x() + translation.x * Float4::unit_w(),
            Float4::unit_y() + translation.y * Float4::unit_w(),
            Float4::unit_z() + translation.z * Float4::unit_w(),
            Float4::unit_w(),
        )
    }

    pub fn rotation_x(angle: f32) -> Self {
        Self::new(
            Float4::unit_x(),
            Float4::new(0.0, angle.cos(), -(angle.sin()), 0.0),
            Float4::new(0.0, angle.sin(), angle.cos(), 0.0),
            Float4::unit_w(),
        )
    }

    pub fn rotation_y(angle: f32) -> Self {
        Self::new(
            Float4::new(angle.cos(), 0.0, angle.sin(), 0.0),
            Float4::unit_y(),
            Float4::new(-(angle.sin()), 0.0, angle.cos(), 0.0),
            Float4::unit_w(),
        )
    }

    pub fn rotation_z(angle: f32) -> Self {
        Self::new(
            Float4::new(angle.cos(), -(angle.sin()), 0.0, 0.0),
            Float4::new(angle.sin(), angle.cos(), 0.0, 0.0),
            Float4::unit_z(),
            Float4::unit_w(),
        )
    }

    pub fn scaling(scale: Float3) -> Self {
        Self::new(
            scale.x * Float4::unit_x(),
            scale.y * Float4::unit_y(),
            scale.z * Float4::unit_z(),
            Float4::unit_w(),
        )
    }

    pub fn perspective_projection(
        near: f32,
        far: f32,
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
    ) -> Self {
        Self::new(
            Float4::new(
                2.0 * near / (right - left),
                0.0,
                -(right + left) / (right - left),
                0.0,
            ),
            Float4::new(
                0.0,
                2.0 * near / (top - bottom),
                -(top + bottom) / (top - bottom),
                0.0,
            ),
            Float4::new(
                0.0,
                0.0,
                (far + near) / (far - near),
                -2.0 * far * near / (far - near),
            ),
            Float4::new(0.0, 0.0, 1.0, 0.0),
        )
    }

    pub fn transpose(&self) -> Self {
        Self::new(
            Float4::new(self.r1.x, self.r2.x, self.r3.x, self.r4.x),
            Float4::new(self.r1.y, self.r2.y, self.r3.y, self.r4.y),
            Float4::new(self.r1.z, self.r2.z, self.r3.z, self.r4.z),
            Float4::new(self.r1.w, self.r2.w, self.r3.w, self.r4.w),
        )
    }

    pub fn transform(
        right: Float3,
        up: Float3,
        forward: Float3,
        translation: Float3,
        scale: Float3,
    ) -> Self {
        // Self {
        //     r1: Float4::new(1.0, 0.0, 0.0, translation.x),
        //     r2: Float4::new(0.0, 1.0, 0.0, translation.y),
        //     r3: Float4::new(0.0, 0.0, 1.0, translation.z),
        //     r4: Float4::new(0.0, 0.0, 0.0, 1.0),
        // } * Self {
        //     r1: Float4::new(right.x, right.y, right.z, 0.0),
        //     r2: Float4::new(up.x, up.y, up.z, 0.0),
        //     r3: Float4::new(forward.x, forward.y, forward.z, 0.0),
        //     r4: Float4::new(0.0, 0.0, 0.0, 1.0),
        // } * Self {
        //     r1: Float4::new(scale.x, 0.0, 0.0, 0.0),
        //     r2: Float4::new(0.0, scale.y, 0.0, 0.0),
        //     r3: Float4::new(0.0, 0.0, scale.z, 0.0),
        //     r4: Float4::new(0.0, 0.0, 0.0, 1.0),
        // }
        Self {
            r1: Float4::new(
                scale.x * right.x,
                scale.y * right.y,
                scale.z * right.z,
                translation.x,
            ),
            r2: Float4::new(
                scale.x * up.x,
                scale.y * up.y,
                scale.z * up.z,
                translation.y,
            ),
            r3: Float4::new(
                scale.x * forward.x,
                scale.y * forward.y,
                scale.z * forward.z,
                translation.z,
            ),
            r4: Float4::unit_w(),
        }
    }

    pub fn rotate_scale_translate(
        yaw: f32,
        pitch: f32,
        roll: f32,
        translation: Float3,
        scale: Float3,
    ) -> Float4x4 {
        Float4x4::new(
            Float4::new(
                scale.x * roll.cos() * yaw.cos() - roll.sin() * pitch.sin() * yaw.sin(),
                -roll.sin() * pitch.cos(),
                roll.cos() * yaw.sin() + roll.sin() * pitch.sin() * yaw.cos(),
                translation.x,
            ),
            Float4::new(
                roll.sin() * yaw.cos() + roll.cos() * pitch.sin() * yaw.sin(),
                scale.y * roll.cos() * pitch.cos(),
                roll.sin() * yaw.sin() - roll.cos() * pitch.sin() * yaw.cos(),
                translation.y,
            ),
            Float4::new(
                -pitch.cos() * yaw.sin(),
                pitch.sin(),
                scale.z * pitch.cos() * yaw.cos(),
                translation.z,
            ),
            Float4::unit_w(),
        )
    }
}

impl Add for Float4x4 {
    type Output = Float4x4;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r1: self.r1 + rhs.r1,
            r2: self.r2 + rhs.r2,
            r3: self.r3 + rhs.r3,
            r4: self.r4 + rhs.r4,
        }
    }
}

impl Mul for Float4x4 {
    type Output = Float4x4;

    fn mul(self, rhs: Self) -> Self::Output {
        Float4x4 {
            r1: Float4::new(
                self.r1.x * rhs.r1.x
                    + self.r1.y * rhs.r2.x
                    + self.r1.z * rhs.r3.x
                    + self.r1.w * rhs.r4.x,
                self.r1.x * rhs.r1.y
                    + self.r1.y * rhs.r2.y
                    + self.r1.z * rhs.r3.y
                    + self.r1.w * rhs.r4.y,
                self.r1.x * rhs.r1.z
                    + self.r1.y * rhs.r2.z
                    + self.r1.z * rhs.r3.z
                    + self.r1.w * rhs.r4.z,
                self.r1.x * rhs.r1.w
                    + self.r1.y * rhs.r2.w
                    + self.r1.z * rhs.r3.w
                    + self.r1.w * rhs.r4.w,
            ),
            r2: Float4::new(
                self.r2.x * rhs.r1.x
                    + self.r2.y * rhs.r2.x
                    + self.r2.z * rhs.r3.x
                    + self.r2.w * rhs.r4.x,
                self.r2.x * rhs.r1.y
                    + self.r2.y * rhs.r2.y
                    + self.r2.z * rhs.r3.y
                    + self.r2.w * rhs.r4.y,
                self.r2.x * rhs.r1.z
                    + self.r2.y * rhs.r2.z
                    + self.r2.z * rhs.r3.z
                    + self.r2.w * rhs.r4.z,
                self.r2.x * rhs.r1.w
                    + self.r2.y * rhs.r2.w
                    + self.r2.z * rhs.r3.w
                    + self.r2.w * rhs.r4.w,
            ),
            r3: Float4::new(
                self.r3.x * rhs.r1.x
                    + self.r3.y * rhs.r2.x
                    + self.r3.z * rhs.r3.x
                    + self.r3.w * rhs.r4.x,
                self.r3.x * rhs.r1.y
                    + self.r3.y * rhs.r2.y
                    + self.r3.z * rhs.r3.y
                    + self.r3.w * rhs.r4.y,
                self.r3.x * rhs.r1.z
                    + self.r3.y * rhs.r2.z
                    + self.r3.z * rhs.r3.z
                    + self.r3.w * rhs.r4.z,
                self.r3.x * rhs.r1.w
                    + self.r3.y * rhs.r2.w
                    + self.r3.z * rhs.r3.w
                    + self.r3.w * rhs.r4.w,
            ),
            r4: Float4::new(
                self.r4.x * rhs.r1.x
                    + self.r4.y * rhs.r2.x
                    + self.r4.z * rhs.r3.x
                    + self.r4.w * rhs.r4.x,
                self.r4.x * rhs.r1.y
                    + self.r4.y * rhs.r2.y
                    + self.r4.z * rhs.r3.y
                    + self.r4.w * rhs.r4.y,
                self.r4.x * rhs.r1.z
                    + self.r4.y * rhs.r2.z
                    + self.r4.z * rhs.r3.z
                    + self.r4.w * rhs.r4.z,
                self.r4.x * rhs.r1.w
                    + self.r4.y * rhs.r2.w
                    + self.r4.z * rhs.r3.w
                    + self.r4.w * rhs.r4.w,
            ),
        }
    }
}

impl Mul<Float4x4> for &Float4x4 {
    type Output = Float4x4;

    fn mul(self, rhs: Float4x4) -> Self::Output {
        Float4x4 {
            r1: Float4::new(
                self.r1.x * rhs.r1.x
                    + self.r1.y * rhs.r2.x
                    + self.r1.z * rhs.r3.x
                    + self.r1.w * rhs.r4.x,
                self.r1.x * rhs.r1.y
                    + self.r1.y * rhs.r2.y
                    + self.r1.z * rhs.r3.y
                    + self.r1.w * rhs.r4.y,
                self.r1.x * rhs.r1.z
                    + self.r1.y * rhs.r2.z
                    + self.r1.z * rhs.r3.z
                    + self.r1.w * rhs.r4.z,
                self.r1.x * rhs.r1.w
                    + self.r1.y * rhs.r2.w
                    + self.r1.z * rhs.r3.w
                    + self.r1.w * rhs.r4.w,
            ),
            r2: Float4::new(
                self.r2.x * rhs.r1.x
                    + self.r2.y * rhs.r2.x
                    + self.r2.z * rhs.r3.x
                    + self.r2.w * rhs.r4.x,
                self.r2.x * rhs.r1.y
                    + self.r2.y * rhs.r2.y
                    + self.r2.z * rhs.r3.y
                    + self.r2.w * rhs.r4.y,
                self.r2.x * rhs.r1.z
                    + self.r2.y * rhs.r2.z
                    + self.r2.z * rhs.r3.z
                    + self.r2.w * rhs.r4.z,
                self.r2.x * rhs.r1.w
                    + self.r2.y * rhs.r2.w
                    + self.r2.z * rhs.r3.w
                    + self.r2.w * rhs.r4.w,
            ),
            r3: Float4::new(
                self.r3.x * rhs.r1.x
                    + self.r3.y * rhs.r2.x
                    + self.r3.z * rhs.r3.x
                    + self.r3.w * rhs.r4.x,
                self.r3.x * rhs.r1.y
                    + self.r3.y * rhs.r2.y
                    + self.r3.z * rhs.r3.y
                    + self.r3.w * rhs.r4.y,
                self.r3.x * rhs.r1.z
                    + self.r3.y * rhs.r2.z
                    + self.r3.z * rhs.r3.z
                    + self.r3.w * rhs.r4.z,
                self.r3.x * rhs.r1.w
                    + self.r3.y * rhs.r2.w
                    + self.r3.z * rhs.r3.w
                    + self.r3.w * rhs.r4.w,
            ),
            r4: Float4::new(
                self.r4.x * rhs.r1.x
                    + self.r4.y * rhs.r2.x
                    + self.r4.z * rhs.r3.x
                    + self.r4.w * rhs.r4.x,
                self.r4.x * rhs.r1.y
                    + self.r4.y * rhs.r2.y
                    + self.r4.z * rhs.r3.y
                    + self.r4.w * rhs.r4.y,
                self.r4.x * rhs.r1.z
                    + self.r4.y * rhs.r2.z
                    + self.r4.z * rhs.r3.z
                    + self.r4.w * rhs.r4.z,
                self.r4.x * rhs.r1.w
                    + self.r4.y * rhs.r2.w
                    + self.r4.z * rhs.r3.w
                    + self.r4.w * rhs.r4.w,
            ),
        }
    }
}

impl Mul<&Float4x4> for Float4x4 {
    type Output = Float4x4;

    fn mul(self, rhs: &Float4x4) -> Self::Output {
        Float4x4 {
            r1: Float4::new(
                self.r1.x * rhs.r1.x
                    + self.r1.y * rhs.r2.x
                    + self.r1.z * rhs.r3.x
                    + self.r1.w * rhs.r4.x,
                self.r1.x * rhs.r1.y
                    + self.r1.y * rhs.r2.y
                    + self.r1.z * rhs.r3.y
                    + self.r1.w * rhs.r4.y,
                self.r1.x * rhs.r1.z
                    + self.r1.y * rhs.r2.z
                    + self.r1.z * rhs.r3.z
                    + self.r1.w * rhs.r4.z,
                self.r1.x * rhs.r1.w
                    + self.r1.y * rhs.r2.w
                    + self.r1.z * rhs.r3.w
                    + self.r1.w * rhs.r4.w,
            ),
            r2: Float4::new(
                self.r2.x * rhs.r1.x
                    + self.r2.y * rhs.r2.x
                    + self.r2.z * rhs.r3.x
                    + self.r2.w * rhs.r4.x,
                self.r2.x * rhs.r1.y
                    + self.r2.y * rhs.r2.y
                    + self.r2.z * rhs.r3.y
                    + self.r2.w * rhs.r4.y,
                self.r2.x * rhs.r1.z
                    + self.r2.y * rhs.r2.z
                    + self.r2.z * rhs.r3.z
                    + self.r2.w * rhs.r4.z,
                self.r2.x * rhs.r1.w
                    + self.r2.y * rhs.r2.w
                    + self.r2.z * rhs.r3.w
                    + self.r2.w * rhs.r4.w,
            ),
            r3: Float4::new(
                self.r3.x * rhs.r1.x
                    + self.r3.y * rhs.r2.x
                    + self.r3.z * rhs.r3.x
                    + self.r3.w * rhs.r4.x,
                self.r3.x * rhs.r1.y
                    + self.r3.y * rhs.r2.y
                    + self.r3.z * rhs.r3.y
                    + self.r3.w * rhs.r4.y,
                self.r3.x * rhs.r1.z
                    + self.r3.y * rhs.r2.z
                    + self.r3.z * rhs.r3.z
                    + self.r3.w * rhs.r4.z,
                self.r3.x * rhs.r1.w
                    + self.r3.y * rhs.r2.w
                    + self.r3.z * rhs.r3.w
                    + self.r3.w * rhs.r4.w,
            ),
            r4: Float4::new(
                self.r4.x * rhs.r1.x
                    + self.r4.y * rhs.r2.x
                    + self.r4.z * rhs.r3.x
                    + self.r4.w * rhs.r4.x,
                self.r4.x * rhs.r1.y
                    + self.r4.y * rhs.r2.y
                    + self.r4.z * rhs.r3.y
                    + self.r4.w * rhs.r4.y,
                self.r4.x * rhs.r1.z
                    + self.r4.y * rhs.r2.z
                    + self.r4.z * rhs.r3.z
                    + self.r4.w * rhs.r4.z,
                self.r4.x * rhs.r1.w
                    + self.r4.y * rhs.r2.w
                    + self.r4.z * rhs.r3.w
                    + self.r4.w * rhs.r4.w,
            ),
        }
    }
}

impl<'a, 'b> Mul<&'a Float4x4> for &'b Float4x4 {
    type Output = Float4x4;

    fn mul(self, rhs: &'a Float4x4) -> Self::Output {
        Float4x4 {
            r1: Float4::new(
                self.r1.x * rhs.r1.x
                    + self.r1.y * rhs.r2.x
                    + self.r1.z * rhs.r3.x
                    + self.r1.w * rhs.r4.x,
                self.r1.x * rhs.r1.y
                    + self.r1.y * rhs.r2.y
                    + self.r1.z * rhs.r3.y
                    + self.r1.w * rhs.r4.y,
                self.r1.x * rhs.r1.z
                    + self.r1.y * rhs.r2.z
                    + self.r1.z * rhs.r3.z
                    + self.r1.w * rhs.r4.z,
                self.r1.x * rhs.r1.w
                    + self.r1.y * rhs.r2.w
                    + self.r1.z * rhs.r3.w
                    + self.r1.w * rhs.r4.w,
            ),
            r2: Float4::new(
                self.r2.x * rhs.r1.x
                    + self.r2.y * rhs.r2.x
                    + self.r2.z * rhs.r3.x
                    + self.r2.w * rhs.r4.x,
                self.r2.x * rhs.r1.y
                    + self.r2.y * rhs.r2.y
                    + self.r2.z * rhs.r3.y
                    + self.r2.w * rhs.r4.y,
                self.r2.x * rhs.r1.z
                    + self.r2.y * rhs.r2.z
                    + self.r2.z * rhs.r3.z
                    + self.r2.w * rhs.r4.z,
                self.r2.x * rhs.r1.w
                    + self.r2.y * rhs.r2.w
                    + self.r2.z * rhs.r3.w
                    + self.r2.w * rhs.r4.w,
            ),
            r3: Float4::new(
                self.r3.x * rhs.r1.x
                    + self.r3.y * rhs.r2.x
                    + self.r3.z * rhs.r3.x
                    + self.r3.w * rhs.r4.x,
                self.r3.x * rhs.r1.y
                    + self.r3.y * rhs.r2.y
                    + self.r3.z * rhs.r3.y
                    + self.r3.w * rhs.r4.y,
                self.r3.x * rhs.r1.z
                    + self.r3.y * rhs.r2.z
                    + self.r3.z * rhs.r3.z
                    + self.r3.w * rhs.r4.z,
                self.r3.x * rhs.r1.w
                    + self.r3.y * rhs.r2.w
                    + self.r3.z * rhs.r3.w
                    + self.r3.w * rhs.r4.w,
            ),
            r4: Float4::new(
                self.r4.x * rhs.r1.x
                    + self.r4.y * rhs.r2.x
                    + self.r4.z * rhs.r3.x
                    + self.r4.w * rhs.r4.x,
                self.r4.x * rhs.r1.y
                    + self.r4.y * rhs.r2.y
                    + self.r4.z * rhs.r3.y
                    + self.r4.w * rhs.r4.y,
                self.r4.x * rhs.r1.z
                    + self.r4.y * rhs.r2.z
                    + self.r4.z * rhs.r3.z
                    + self.r4.w * rhs.r4.z,
                self.r4.x * rhs.r1.w
                    + self.r4.y * rhs.r2.w
                    + self.r4.z * rhs.r3.w
                    + self.r4.w * rhs.r4.w,
            ),
        }
    }
}

impl Mul<Float4> for Float4x4 {
    type Output = Float4;

    fn mul(self, rhs: Float4) -> Self::Output {
        Float4 {
            x: self.r1.dot(&rhs),
            y: self.r2.dot(&rhs),
            z: self.r3.dot(&rhs),
            w: self.r4.dot(&rhs),
        }
    }
}

impl Mul<Float4> for &Float4x4 {
    type Output = Float4;

    fn mul(self, rhs: Float4) -> Self::Output {
        Float4 {
            x: self.r1.dot(&rhs),
            y: self.r2.dot(&rhs),
            z: self.r3.dot(&rhs),
            w: self.r4.dot(&rhs),
        }
    }
}

impl Mul<&Float4> for &Float4x4 {
    type Output = Float4;

    fn mul(self, rhs: &Float4) -> Self::Output {
        Float4 {
            x: self.r1.dot(rhs),
            y: self.r2.dot(rhs),
            z: self.r3.dot(rhs),
            w: self.r4.dot(rhs),
        }
    }
}

// pub fn point_on_right_side_of_line(a: Float2, b: Float2, p: Float2) -> bool {
//     let ap = p - a;
//     let ab_perp = (b - a).perp();
//     ap.dot(ab_perp) >= 0.0
// }

// pub fn point_in_triangle(a: Float2, b: Float2, c: Float2, p: Float2) -> bool {
//     let side_ab = point_on_right_side_of_line(a, b, p);
//     let side_bc = point_on_right_side_of_line(b, c, p);
//     let side_ca = point_on_right_side_of_line(c, a, p);
//     side_ab && side_bc && side_ca
// }

// calculates area of triangle ABC (positive if clockwise, otherwise negative)
pub fn signed_triangle_area(a: Float2, b: Float2, c: Float2) -> f32 {
    let ac = c - a;
    let ab_perp = (b - a).perp();
    ac.dot(ab_perp) / 2.0
}

pub fn point_in_triangle(a: Float2, b: Float2, c: Float2, p: Float2) -> Option<Float3> {
    // Test if point is on right side of each edge segment
    let area_abp = signed_triangle_area(a, b, p);
    let area_bcp = signed_triangle_area(b, c, p);
    let area_cap = signed_triangle_area(c, a, p);

    // Weighting factors (barycentric coordinates)
    let total_area = area_abp + area_bcp + area_cap;
    let inverse_area_sum = 1.0 / total_area;
    let weight_a = area_bcp * inverse_area_sum;
    let weight_b = area_cap * inverse_area_sum;
    let weight_c = area_abp * inverse_area_sum;

    if total_area > 0.0 && area_abp >= 0.0 && area_bcp >= 0.0 && area_cap >= 0.0 {
        Some(Float3::new(weight_a, weight_b, weight_c))
    } else {
        None
    }
}

pub fn barycentric_coords(a: Float2, b: Float2, c: Float2, p: Float2) -> Option<Float3> {
    let (v0, v1, v2) = (b - a, c - a, p - a);
    let d00 = v0.dot(v0);
    let d01 = v0.dot(v1);
    let d11 = v1.dot(v1);
    let d20 = v2.dot(v0);
    let d21 = v2.dot(v1);
    let denom = d00 * d11 - d01 * d01;
    if denom.abs() <= f32::EPSILON {
        None
    } else {
        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;

        Some(Float3::new(u, v, w))
    }
}
