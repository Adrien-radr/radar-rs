use std::ops::{Add, Mul, Div, Sub, DivAssign, MulAssign, SubAssign, AddAssign, Index, IndexMut};
use std::fmt;


use math::vec4::*;

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Mat4 {
    pub m: [Vec4; 4],
}

impl fmt::Display for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Mat4({},{},{},{})",
               self.m[0],
               self.m[1],
               self.m[2],
               self.m[3])
    }
}

impl Add<Mat4> for Mat4 {
    type Output = Mat4;
    #[inline(always)]
    fn add(self, other: Mat4) -> Mat4 {
        Mat4 {
            m: [self.m[0] + other.m[0],
                self.m[1] + other.m[1],
                self.m[2] + other.m[2],
                self.m[3] + other.m[3]],
        }
    }
}

impl AddAssign<Mat4> for Mat4 {
    #[inline(always)]
    fn add_assign(&mut self, other: Mat4) {
        self.m[0] += other.m[0];
        self.m[1] += other.m[1];
        self.m[2] += other.m[2];
        self.m[3] += other.m[3];
    }
}

impl Sub<Mat4> for Mat4 {
    type Output = Mat4;
    #[inline(always)]
    fn sub(self, other: Mat4) -> Mat4 {
        Mat4 {
            m: [self.m[0] - other.m[0],
                self.m[1] - other.m[1],
                self.m[2] - other.m[2],
                self.m[3] - other.m[3]],
        }
    }
}

impl SubAssign<Mat4> for Mat4 {
    fn sub_assign(&mut self, other: Mat4) {
        self.m[0] -= other.m[0];
        self.m[1] -= other.m[1];
        self.m[2] -= other.m[2];
        self.m[3] -= other.m[3];
    }
}

/// TODO maybe refactor this function or the undergoing storage of Mat4 to allow faster multiplication
impl Mul<Mat4> for Mat4 {
    type Output = Mat4;
    #[inline(always)]
    fn mul(self, other: Mat4) -> Mat4 {
        Mat4 {
            m: [Vec4::new(self.m[0].x * other.m[0].x,
                          self.m[1].x * other.m[0].y,
                          self.m[2].x * other.m[0].z,
                          self.m[3].x * other.m[0].w),
                Vec4::new(self.m[0].y * other.m[1].x,
                          self.m[1].y * other.m[1].y,
                          self.m[2].y * other.m[1].z,
                          self.m[3].y * other.m[1].w),
                Vec4::new(self.m[0].z * other.m[2].x,
                          self.m[1].z * other.m[2].y,
                          self.m[2].z * other.m[2].z,
                          self.m[3].z * other.m[2].w),
                Vec4::new(self.m[0].w * other.m[3].x,
                          self.m[1].w * other.m[3].y,
                          self.m[2].w * other.m[3].z,
                          self.m[3].w * other.m[3].w)],
        }
    }
}

/// scalar multiplication of a matrix
impl Mul<f32> for Mat4 {
    type Output = Mat4;
    #[inline(always)]
    fn mul(self, other: f32) -> Mat4 {
        Mat4 { m: [self.m[0] * other, self.m[1] * other, self.m[2] * other, self.m[3] * other] }
    }
}

impl MulAssign<f32> for Mat4 {
    #[inline(always)]
    fn mul_assign(&mut self, other: f32) {
        self.m[0] *= other;
        self.m[1] *= other;
        self.m[2] *= other;
        self.m[3] *= other;
    }
}

impl MulAssign<Mat4> for Mat4 {
    #[inline(always)]
    fn mul_assign(&mut self, other: Mat4) {
        self.m[0].x *= other.m[0].x;
        self.m[1].x *= other.m[0].y;
        self.m[2].x *= other.m[0].z;
        self.m[3].x *= other.m[0].w;
        self.m[0].y *= other.m[1].x;
        self.m[1].y *= other.m[1].y;
        self.m[2].y *= other.m[1].z;
        self.m[3].y *= other.m[1].w;
        self.m[0].z *= other.m[2].x;
        self.m[1].z *= other.m[2].y;
        self.m[2].z *= other.m[2].z;
        self.m[3].z *= other.m[2].w;
        self.m[0].w *= other.m[3].x;
        self.m[1].w *= other.m[3].y;
        self.m[2].w *= other.m[3].z;
        self.m[3].w *= other.m[3].w;
    }
}

/// Read only access to specified column of the matrix
impl Index<usize> for Mat4 {
    type Output = Vec4;
    fn index<'a>(&'a self, column: usize) -> &'a Vec4 {
        match column {
            i @ 0...4 => &self.m[i],
            _ => panic!("Invalid index access in Mat4 {:?}"),
        }
    }
}

impl IndexMut<usize> for Mat4 {  
    fn index_mut<'a>(&'a mut self, _index :usize) -> &'a mut Vec4 {
        match _index{
            i @ 0...4 => &mut self.m[i],
            _ => panic!("Invalid index access in Mat4 {:?}"),            
        }
    }
}

impl Div<Mat4> for Mat4 {
    type Output = Mat4;
    fn div(self, other : Mat4) -> Mat4 {
        self * Mat4::inverse(other)
    }
}

impl Div<f32> for Mat4 {
    type Output = Mat4;
    fn div(self, other : f32) -> Mat4{
        Mat4{
            m:[
                self[0] / other,
                self[1] / other,
                self[2] / other,
                self[3] / other,
            ]
        }
    }
}

impl DivAssign<Mat4> for Mat4 {
    fn div_assign(&mut self, other : Mat4){
        *self *= Mat4::inverse(other);
    }
}

impl DivAssign<f32> for Mat4 {
    fn div_assign(&mut self, other : f32){
        *self *= other;
    }
}

impl Mat4 {
    pub fn new(a11: f32,
               a21: f32,
               a31: f32,
               a41: f32,
               a12: f32,
               a22: f32,
               a32: f32,
               a42: f32,
               a13: f32,
               a23: f32,
               a33: f32,
               a43: f32,
               a14: f32,
               a24: f32,
               a34: f32,
               a44: f32)
               -> Mat4 {
        Mat4 {
            m: [Vec4::new(a11, a12, a13, a14),
                Vec4::new(a21, a22, a23, a24),
                Vec4::new(a31, a32, a33, a34),
                Vec4::new(a41, a42, a43, a44)],
        }

    }

    pub fn new_from_vec4(col1: Vec4, col2: Vec4, col3: Vec4, col4: Vec4) -> Mat4 {
        Mat4 { m: [col1, col2, col3, col4] }
    }

    pub fn empty() -> Mat4{
        Mat4{
            m:[
                Vec4::empty(),
                Vec4::empty(),
                Vec4::empty(),
                Vec4::empty(),
            ]
        }
    }

    pub fn identity() -> Mat4 {
        Mat4 {
            m: [Vec4::new(1.0, 0.0, 0.0, 0.0),
                Vec4::new(0.0, 1.0, 0.0, 0.0),
                Vec4::new(0.0, 0.0, 1.0, 0.0),
                Vec4::new(0.0, 0.0, 0.0, 1.0)],
        }
    }

    pub fn inverse(m : Mat4) -> Mat4 {
        let mut r = Mat4::empty();
        r[0][0] = m[1][1] * ( m[2][2] * m[3][3] - m[2][3] * m[3][2] ) - m[2][1] * ( m[1][2] * m[3][3] - m[1][3] * m[3][2] ) - m[3][1] * ( m[1][3] * m[2][2] - m[1][2] * m[2][3] );
		r[0][1] = m[0][1] * ( m[2][3] * m[3][2] - m[2][2] * m[3][3] ) - m[2][1] * ( m[0][3] * m[3][2] - m[0][2] * m[3][3] ) - m[3][1] * ( m[0][2] * m[2][3] - m[0][3] * m[2][2] );
		r[0][2] = m[0][1] * ( m[1][2] * m[3][3] - m[1][3] * m[3][2] ) - m[1][1] * ( m[0][2] * m[3][3] - m[0][3] * m[3][2] ) - m[3][1] * ( m[0][3] * m[1][2] - m[0][2] * m[1][3] );
		r[0][3] = m[0][1] * ( m[1][3] * m[2][2] - m[1][2] * m[2][3] ) - m[1][1] * ( m[0][3] * m[2][2] - m[0][2] * m[2][3] ) - m[2][1] * ( m[0][2] * m[1][3] - m[0][3] * m[1][2] );

		r[1][0] = m[1][0] * ( m[2][3] * m[3][2] - m[2][2] * m[3][3] ) - m[2][0] * ( m[1][3] * m[3][2] - m[1][2] * m[3][3] ) - m[3][0] * ( m[1][2] * m[2][3] - m[1][3] * m[2][2] );
		r[1][1] = m[0][0] * ( m[2][2] * m[3][3] - m[2][3] * m[3][2] ) - m[2][0] * ( m[0][2] * m[3][3] - m[0][3] * m[3][2] ) - m[3][0] * ( m[0][3] * m[2][2] - m[0][2] * m[2][3] );
		r[1][2] = m[0][0] * ( m[1][3] * m[3][2] - m[1][2] * m[3][3] ) - m[1][0] * ( m[0][3] * m[3][2] - m[0][2] * m[3][3] ) - m[3][0] * ( m[0][2] * m[1][3] - m[0][3] * m[1][2] );
		r[1][3] = m[0][0] * ( m[1][2] * m[2][3] - m[1][3] * m[2][2] ) - m[1][0] * ( m[0][2] * m[2][3] - m[0][3] * m[2][2] ) - m[2][0] * ( m[0][3] * m[1][2] - m[0][2] * m[1][3] );

		r[2][0] = m[1][0] * ( m[2][1] * m[3][3] - m[2][3] * m[3][1] ) - m[2][0] * ( m[1][1] * m[3][3] - m[1][3] * m[3][1] ) - m[3][0] * ( m[1][3] * m[2][1] - m[1][1] * m[2][3] );
		r[2][1] = m[0][0] * ( m[2][3] * m[3][1] - m[2][1] * m[3][3] ) - m[2][0] * ( m[0][3] * m[3][1] - m[0][1] * m[3][3] ) - m[3][0] * ( m[0][1] * m[2][3] - m[0][3] * m[2][1] );
		r[2][2] = m[0][0] * ( m[1][1] * m[3][3] - m[1][3] * m[3][1] ) - m[1][0] * ( m[0][1] * m[3][3] - m[0][3] * m[3][1] ) - m[3][0] * ( m[0][3] * m[1][1] - m[0][1] * m[1][3] );
		r[2][3] = m[0][0] * ( m[1][3] * m[2][1] - m[1][1] * m[2][3] ) - m[1][0] * ( m[0][3] * m[2][1] - m[0][1] * m[2][3] ) - m[2][0] * ( m[0][1] * m[1][3] - m[0][3] * m[1][1] );

		r[3][0] = m[1][0] * ( m[2][2] * m[3][1] - m[2][1] * m[3][2] ) - m[2][0] * ( m[1][2] * m[3][1] - m[1][1] * m[3][2] ) - m[3][0] * ( m[1][1] * m[2][2] - m[1][2] * m[2][1] );
		r[3][1] = m[0][0] * ( m[2][1] * m[3][2] - m[2][2] * m[3][1] ) - m[2][0] * ( m[0][1] * m[3][2] - m[0][2] * m[3][1] ) - m[3][0] * ( m[0][2] * m[2][1] - m[0][1] * m[2][2] );
		r[3][2] = m[0][0] * ( m[1][2] * m[3][1] - m[1][1] * m[3][2] ) - m[1][0] * ( m[0][2] * m[3][1] - m[0][1] * m[3][2] ) - m[3][0] * ( m[0][1] * m[1][2] - m[0][2] * m[1][1] );
		r[3][3] = m[0][0] * ( m[1][1] * m[2][2] - m[1][2] * m[2][1] ) - m[1][0] * ( m[0][1] * m[2][2] - m[0][2] * m[2][1] ) - m[2][0] * ( m[0][2] * m[1][1] - m[0][1] * m[1][2] );
        r
    }
}