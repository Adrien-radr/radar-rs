mod context;
mod shader;
mod system;
mod math;

extern crate gl;
use gl::types::*;
use std::mem;
use std::ptr;
use std::ffi::CString;

use context::Context;

// Vertex data
static VERTEX_DATA: [GLfloat; 6] = [
     0.0,  0.5,
     0.5, -0.5,
    -0.5, -0.5
];

// Shader sources
static VS_SRC: &'static str =
   "#version 400\n\
    in vec2 position;\n\
    void main() {\n\
       gl_Position = vec4(position, 0.0, 1.0);\n\
    }";

static FS_SRC: &'static str =
   "#version 400\n\
    out vec4 out_color;\n\
    void main() {\n\
       out_color = vec4(1.0, 1.0, 1.0, 1.0);\n\
    }";


fn main() {
    let mut ctx = Context::new("data/config.json");

    let vs = shader::compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = shader::compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = shader::link_program(vs, fs);

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&VERTEX_DATA[0]),
            gl::STATIC_DRAW);

        gl::UseProgram(program);
        gl::BindFragDataLocation(program, 0,
            CString::new("out_color").unwrap().as_ptr());

        let pos_attr = gl::GetAttribLocation(program,
            CString::new("position").unwrap().as_ptr());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT, 
            gl::FALSE as GLboolean, 0, ptr::null());
    }

    while ctx.is_running() {
        ctx.start_frame();

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        ctx.end_frame();
    }

    // free mem
    unsafe {
        gl::DeleteProgram(program);
        gl::DeleteShader(vs);
        gl::DeleteShader(fs);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}
