use std::ops::{Add, Mul, Div, Sub, DivAssign, MulAssign, SubAssign, AddAssign, Neg, Index,
               IndexMut};
use std::fmt;

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Index<usize> for Vec4 {
    type Output = f32;
    fn index<'a>(&'a self, _index: usize) -> &'a f32 {
        match _index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,   
            _ => panic!("Invalid index access in Vec4 {:?}"),
        }
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut<'a>(&'a mut self, _index: usize) -> &'a mut f32 {
        match _index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,   
            _ => panic!("Invalid index access in Vec4 {:?}"),
        }
    }
}

// impl Index<usize> for Vec4 {
//     type Output = f32;
//     fn index<'a>(&'a self, index: usize) -> &'a f32 {
//         match index {
//             0 => self.x,
//             1 => self.y,
//             2 => self.z,
//             _ => panic!("Invalid index for Vec4 access {:?}"),
//         }
//     }
// }

/// Display function
impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{},{})", self.x, self.y, self.z, self.w)
    }
}

/// Inverse a vector
impl Neg for Vec4 {
    type Output = Vec4;
    fn neg(self) -> Vec4 {
        Vec4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
}

/// Simple addition between two vectors (component-wise)
impl Add for Vec4 {
    type Output = Vec4;
    #[inline(always)]
    fn add(self, other: Vec4) -> Vec4 {
        Vec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

/// Simple self-addition
impl AddAssign for Vec4 {
    fn add_assign(&mut self, other: Vec4) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

/// Simple substraction between two vectors (component-wise)
impl Sub for Vec4 {
    type Output = Vec4;
    #[inline(always)]
    fn sub(self, other: Vec4) -> Vec4 {
        Vec4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

/// Simple self substraction
impl SubAssign for Vec4 {
    fn sub_assign(&mut self, other: Vec4) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

/// Simple component-wise multiplication between two vectors
impl Mul<Vec4> for Vec4 {
    type Output = Vec4;
    #[inline(always)]
    fn mul(self, other: Vec4) -> Vec4 {
        Vec4 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

/// Simple component-wise self assign multiplication between two vectors
impl MulAssign<Vec4> for Vec4 {
    #[inline(always)]
    fn mul_assign(&mut self, other: Vec4) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self.w *= other.w;
    }
}

/// Simple component-wise multiplication between two vectors
impl Mul<f32> for Vec4 {
    type Output = Vec4;
    #[inline(always)]
    fn mul(self, other: f32) -> Vec4 {
        Vec4 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

/// Simple component-wise self assign multiplication between two vectors
impl MulAssign<f32> for Vec4 {
    #[inline(always)]
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
        self.w *= other;
    }
}

/// Component wise division a / b
impl Div<Vec4> for Vec4 {
    type Output = Vec4;
    #[inline(always)]
    fn div(self, other: Vec4) -> Vec4 {
        Vec4 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }
}

/// Component wise division a / b
impl DivAssign<Vec4> for Vec4 {
    fn div_assign(&mut self, other: Vec4) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
        self.w /= other.w;
    }
}

/// Scalar division
impl Div<f32> for Vec4 {
    type Output = Vec4;
    #[inline(always)]
    fn div(self, other: f32) -> Vec4 {
        Vec4 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

/// Component wise division a / b
impl DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
        self.w /= other;
    }
}

impl Vec4 {
    /// create a new vector with the specified coordinates
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    /// return an empty vector
    pub fn empty() -> Vec4 {
        Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    /// return an unit vector on x axis
    pub fn x_axis() -> Vec4 {
        Vec4 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    /// return an unit vector on y axis
    pub fn y_axis() -> Vec4 {
        Vec4 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            w: 0.0,
        }
    }

    /// return an unit vector on z axis
    pub fn z_axis() -> Vec4 {
        Vec4 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            w: 0.0,
        }
    }

    pub fn normalize(a: Vec4) -> Vec4 {
        let len = Vec4::length(a);
        if len > 0.0 {
            let k = 1.0 / len;
            a * k
        } else {
            Vec4::empty()
        }
    }

    pub fn dot(a: Vec4, b: Vec4) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }

    // pub fn cross(a: Vec4, b: Vec4) -> Vec4 {
    //     unimplemented!();
    // }

    pub fn length(a: Vec4) -> f32 {
        let dot_product = Vec4::dot(a, a);
        if dot_product > 0.0 {
            dot_product.sqrt()
        } else {
            0.0
        }
    }
}
