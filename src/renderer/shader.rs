extern crate gl;

use system::filesystem;
use math::vec3::*;
use math::vec4::*;
use math::mat4::*;

use self::gl::types::*;
use std::ffi::CString;
use std::ptr;
use std::str;
use std::collections::HashMap;
use std::mem;

pub enum ShaderType{
    VERTEX,
    TESSCRTL,
    TESSEVAL,
    GEOMETRY,
    FRAGMENT,
}

impl ShaderType {
    pub fn to_gl_type(&self) -> GLenum {
        match self {
            &ShaderType::VERTEX => gl::VERTEX_SHADER,
            &ShaderType::FRAGMENT => gl::FRAGMENT_SHADER,
            &ShaderType::GEOMETRY => gl::GEOMETRY_SHADER,
            &ShaderType::TESSEVAL => gl::TESS_EVALUATION_SHADER,
            &ShaderType::TESSCRTL => gl::TESS_CONTROL_SHADER,
        }
    }
}

/// Representation of one shader (vertex shader, fragment shader, etc.)
pub struct Shader{
    pub shader_id: GLuint, 
    shader_type: ShaderType,
    source_file: String,
    source: String,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.shader_id); }
    }
}

impl Shader{
    pub fn new(shader_type : ShaderType, sourceFilePath : String) -> Shader {
        let src = filesystem::read_file(&sourceFilePath);
        Shader {
            shader_id : compile_shader(&src, shader_type.to_gl_type()), 
            shader_type : shader_type,
            source_file : sourceFilePath,
            source : src 
        }
    }
}


pub struct Program{
    pub program_id : GLuint,
    uniform_loc : HashMap<String, GLint>,
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.program_id); }
    }
}

impl Program {
    /// Create a GPU Program (unsafe)
    pub fn new() -> Program {
        unsafe{
            Program {
                program_id : gl::CreateProgram(),
                uniform_loc : HashMap::new(),              
            }
        }
    }

    pub fn bind(&self) {
        unsafe{
            gl::UseProgram(self.program_id);
        }
    }

    pub fn register_uniform(&mut self, name: &str) {
        unsafe {
            let loc =  gl::GetUniformLocation(self.program_id, CString::new(name).unwrap().as_ptr());
            self.uniform_loc.insert(name.to_string(), loc);
        }
    }

    fn get_uniform(&self, name: &str) -> GLint {
        match self.uniform_loc.get(name) {
            Some(loc) => return *loc,
            _ => panic!("Uniform {} doesnt exist for shader program {}", name, self.program_id)
        }
    }

    pub fn set_uniform_matrix4fv(&self, name : &str, mat4 : &Mat4){
        let loc = self.get_uniform(name);
        unsafe {
            gl::UniformMatrix4fv(loc, 1, gl::FALSE, mat4.as_ptr());
        }
    }

    pub fn set_uniform_2fv(&self, name: &str, vec: (f32, f32)) {
        let loc = self.get_uniform(name);
        unsafe {
            gl::Uniform2fv(loc, 1, &vec.0 as *const f32);
        }
    }

    pub fn set_uniform_3fv(&self, name: &str, vec: &Vec3) {
        let loc = self.get_uniform(name);
        unsafe {
            gl::Uniform3fv(loc, 1, vec.as_ptr());
        }
    }

    pub fn set_uniform_4fv(&self, name: &str, vec: &Vec4) {
        let loc = self.get_uniform(name);
        unsafe {
            gl::Uniform4fv(loc, 1, vec.as_ptr());
        }
    }

    pub fn set_uniform_1i(&self, name : &str, int : GLint){
        let loc = self.get_uniform(name);
        unsafe {
            gl::Uniform1i(loc, int);
        }
    }

    pub fn attach(&self, shader : &Shader){
        unsafe {
            gl::AttachShader(self.program_id,shader.shader_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn link(&self){
        unsafe{
            gl::LinkProgram(self.program_id);
            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(self.program_id, gl::LINK_STATUS,&mut status);
            if status != (gl::TRUE as GLint) {
                let mut len : GLint = 0;
                gl::GetProgramiv(self.program_id, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize - 1));
                gl::GetProgramInfoLog(self.program_id, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
                panic!("{}", str::from_utf8(&buf).ok().expect("ProgramInfoLog not valid utf8"));             
            }

            self.bind();
            gl::BindFragDataLocation(self.program_id, 0, CString::new("out_color").unwrap().as_ptr());
        }
    }
}


fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);

        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // no trailing char
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8"));
        }
    }
    shader
}