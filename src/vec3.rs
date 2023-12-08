use num_traits::{Float, PrimInt};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}
pub type Color = Vec3;
pub type Point3 = Vec3;


/// https://raytracing.github.io/books/RayTracingInOneWeekend.html#thevec3class
impl Vec3 {
    pub fn new(e1: f64, e2: f64, e3: f64) -> Self {
        Self { e: [e1, e2, e3] }
    }

    pub fn default() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }

    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        return self[0] * self[0]
             + self[1] * self[1]
             + self[2] * self[2];
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        return self[0] * rhs[0]
             + self[1] * rhs[1]
             + self[2] * rhs[2];
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self::new(self[1] * rhs[2] - self[2] - rhs[1],
                  self[2] * rhs[0] - self[0] * rhs[2],
                  self[0] * rhs[1] - self[1] * rhs[0])
    }

    pub fn unit_vector(&self) -> Self {
        let length = self.length();
        Self::new(self[0] / length,
                  self[1] / length,
                  self[2] / length)
    }
}


// Utility functions

impl std::ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self[0] + rhs[0],
                  self[1] + rhs[1],
                  self[2] + rhs[2])
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self[0] - rhs[0],
                  self[1] - rhs[1],
                  self[2] - rhs[2])
    }
}

impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self[0] * rhs[0],
                  self[1] * rhs[1],
                  self[2] * rhs[2])
    }
}

impl std::ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self[0] *= rhs[0];
        self[1] *= rhs[1];
        self[2] *= rhs[2];
    }
}

impl std::ops::Div<Vec3> for Vec3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self::new(self[0] / rhs[0],
                  self[1] / rhs[1],
                  self[2] / rhs[2])
    }
}

impl std::ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self[0] /= rhs[0];
        self[1] /= rhs[1];
        self[2] /= rhs[2];
    }
}

macro_rules! impl_ops {
    ($type:ty) => {
        impl std::ops::Mul<$type> for Vec3 {
            type Output = Self;
            fn mul(self, rhs: $type) -> Self {
                Self::new(self[0] * rhs as f64,
                          self[1] * rhs as f64,
                          self[2] * rhs as f64)
            }
        }

        impl std::ops::Mul<Vec3> for $type {
            type Output = Vec3;
            fn mul(self, rhs: Vec3) -> Vec3 {
                rhs * self
            }
        }

        impl std::ops::MulAssign<$type> for Vec3 {
            fn mul_assign(&mut self, rhs: $type) {
                self[0] *= rhs as f64;
                self[1] *= rhs as f64;
                self[2] *= rhs as f64;
            }
        }

        impl std::ops::Div<$type> for Vec3 {
            type Output = Self;
            fn div(self, rhs: $type) -> Self {
                 Self::new(self[0] / rhs as f64,
                          self[1] / rhs as f64,
                          self[2] / rhs as f64)
            }
        }

        impl std::ops::Div<Vec3> for $type {
            type Output = Vec3;
            fn div(self, rhs: Vec3) -> Vec3 {
                rhs / self
            }
        }

        impl std::ops::DivAssign<$type> for Vec3 {
            fn div_assign(&mut self, rhs: $type) {
                self[0] /= rhs as f64;
                self[1] /= rhs as f64;
                self[2] /= rhs as f64;
            }
        }
    };
}

impl_ops!(i32);
impl_ops!(i64);
impl_ops!(u32);
impl_ops!(u64);
impl_ops!(f32);
impl_ops!(f64);


impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}",
               (255.999 * self[0]).floor() as i32,
               (255.999 * self[1]).floor() as i32,
               (255.999 * self[2]).floor() as i32)
    }
}