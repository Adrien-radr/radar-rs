use math::vec4::*;
use math::vec3::*;
use math::mat4::*;

pub fn translate(pos : Vec3) -> Mat4 {
    Mat4::new_from_vec4(
        Vec4::new(1.0,0.0,0.0,0.0),
        Vec4::new(0.0,1.0,0.0,0.0),
        Vec4::new(0.0,0.0,1.0,0.0),
        Vec4::new(pos.x,pos.y,pos.z,1.0)
    )
}

/// Return a rotation Matrix (Mat4) for a rotation of a particular
/// angle in degree (angle_deg) around an unit axis.
pub fn rotate(angle_deg : f32, axis : Vec3) -> Mat4 {
    let axis = Vec3::normalize(axis);
    let sin = angle_deg.sin();
    let cos = angle_deg.cos();
    let negcos = 1.0 - cos;
    Mat4::new_from_vec4(
        Vec4::new(cos + axis.x.powi(2) * negcos,axis.y * axis.x * negcos + axis.z * sin, axis.z * axis.x * negcos - axis.y * sin, 0.0),
        Vec4::new(axis.x * axis.y * negcos - axis.z * sin,cos + axis.y.powi(2) * negcos, axis.z * axis.y * negcos + axis.x * sin, 0.0),
        Vec4::new(axis.x * axis.z * negcos + axis.y * sin, axis.y * axis.z * negcos - axis.x * sin, cos + axis.z.powi(2) * negcos, 0.0),
        Vec4::new(0.0,0.0,0.0,1.0),
    )
}