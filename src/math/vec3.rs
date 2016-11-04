use std::ops::{Add, Mul, Div, Sub, DivAssign, MulAssign, SubAssign, AddAssign, Neg};
use std::fmt;

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// impl Index<usize> for Vec3 {
//     type Output = f32;
//     fn index<'a>(&'a self, index: usize) -> &'a f32 {
//         match index {
//             0 => self.x,
//             1 => self.y,
//             2 => self.z,
//             _ => panic!("Invalid index for Vec3 access {:?}"),
//         }
//     }
// }

/// Display function
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

/// Inverse a vector
impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

/// Simple addition between two vectors (component-wise)
impl Add for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

/// Simple self-addition
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

/// Simple substraction between two vectors (component-wise)
impl Sub for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// Simple self substraction
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

/// Simple component-wise multiplication between two vectors
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

/// Simple component-wise self assign multiplication between two vectors
impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

/// Simple component-wise multiplication between two vectors
impl Mul<f32> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

/// Simple component-wise self assign multiplication between two vectors
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

/// Component wise division a / b
impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

/// Component wise division a / b
impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

/// Scalar division
impl Div<f32> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn div(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

/// Component wise division a / b
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl Vec3 {
    /// create a new vector with the specified coordinates
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    /// return an empty vector
    pub fn empty() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// return an unit vector on x axis
    pub fn x() -> Vec3 {
        Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// return an unit vector on y axis
    pub fn y() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    /// return an unit vector on z axis
    pub fn z() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    pub fn normalize(a: Vec3) -> Vec3 {
        let length = (a.x * a.x + a.y * a.y + a.z * a.z).sqrt();
        // a / length
        Vec3 {
            x: a.x / length,
            y: a.y / length,
            z: a.z / length,
        }
    }

    pub fn dot(a: Vec3, b: Vec3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        Vec3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.z,
        }
    }

    pub fn length(a: Vec3) -> f32 {
        let dot_product = Vec3::dot(a, a);
        if dot_product > 0.0 {
            dot_product.sqrt()
        } else {
            0.0
        }
    }
}
