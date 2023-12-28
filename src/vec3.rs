#[derive(Clone, Copy, Debug)]
pub(crate) struct Vec3 {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

pub(crate) type Color = Vec3;
pub(crate) type Point3 = Vec3;

impl Vec3 {
    pub(crate) fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub(crate) fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
    pub(crate) fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub(crate) fn length_squared(&self) -> f64 {
        self[0].powi(2) + self[1].powi(2) + self[2].powi(2)
    }
}


// Utility functions

pub(crate) fn unit_vector(v: Vec3) -> Vec3 {
    let length = v.length();
    Vec3::new(v[0] / length,
              v[1] / length,
              v[2] / length)
}

pub(crate) fn dot(lhs: Vec3, rhs: Vec3) -> f64 {
    lhs[0] * rhs[0] + lhs[1] * rhs[1] + lhs[2] * rhs[2]
}

pub(crate) fn cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
    Vec3::new(lhs[1] * rhs[2] - lhs[2] - rhs[1],
              lhs[2] * rhs[0] - lhs[0] * rhs[2],
              lhs[0] * rhs[1] - lhs[1] * rhs[0])
}


// Implementations

impl std::ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds: the length is 3 but the index is {index}"),
        }
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds: the length is 3 but the index is {index}"),
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self[0],
                  -self[1],
                  -self[2])
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
