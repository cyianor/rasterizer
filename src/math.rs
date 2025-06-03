use rand::{
    Rng,
    distr::uniform::{SampleUniform, UniformFloat, UniformSampler},
};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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
pub struct Float2 {
    pub x: f32,
    pub y: f32,
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

impl Mul for Float2 {
    type Output = Self;

    fn mul(self, rhs: Float2) -> Self::Output {
        Self {
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

impl Mul<Float2> for f32 {
    type Output = Float2;

    fn mul(self, rhs: Float2) -> Self::Output {
        Float2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Div<f32> for Float2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
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
