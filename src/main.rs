mod context;
mod shader;
mod system;
mod math;
mod mesh;

extern crate gl;
use gl::types::*;
use std::mem;
use std::ptr;
use std::ffi::CString;

use context::Context;

// Vertex data
static VERTEX_DATA: [GLfloat; 9] = [
     0.0,  0.5, 0.0,
     0.5, -0.5, 0.0,
    -0.5, -0.5, 0.0
];
static VERTEX_COL_DATA: [GLfloat; 12] = [
     1.0, 1.0, 1.0, 1.0,
     1.0, 0.0, 1.0, 1.0,
     0.0, 1.0, 1.0, 1.0
];

// Shader sources
static VS_SRC: &'static str =
   "#version 400\n\
    in vec3 position;\n\
    in vec4 color;\n\
    out vec4 vColor;\n\
    void main() {\n\
        vColor = color;\n\
       gl_Position = vec4(position, 1.0);\n\
    }";

static FS_SRC: &'static str =
   "#version 400\n\
    in vec4 vColor;\n\
    out vec4 out_color;\n\
    void main() {\n\
       out_color = vColor;\n\
    }";


fn main() {
    let mut ctx = Context::new("data/config.json");

    let vs = shader::compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = shader::compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = shader::link_program(vs, fs);

    let m0 = mesh::Mesh::new(&VERTEX_DATA, &VERTEX_COL_DATA);

    unsafe {
        gl::UseProgram(program);
        gl::BindFragDataLocation(program, 0,
            CString::new("out_color").unwrap().as_ptr());
    }

    while ctx.is_running() {
        ctx.start_frame();

        m0.render();

        ctx.end_frame();
    }

    // free mem
    unsafe {
        gl::DeleteProgram(program);
        gl::DeleteShader(vs);
        gl::DeleteShader(fs);
    }
}
