mod system;
mod math;
mod renderer;

use renderer::context::*;
use renderer::mesh;
use renderer::shader;
use renderer::texture;


extern crate rand;
extern crate time;
extern crate gl;

use gl::types::*;
use std::mem;
use std::ptr;
use std::ffi::CString;
use rand::Rng;


// Vertex data
static VERTEX_DATA: [GLfloat; 9] = [
     0.0,  0.5, 0.0,
     0.5, -0.5, 0.0,
    -0.5, -0.5, 0.0
];

static VERTEX_TEX_DATA: [GLfloat; 6] = [
    0.5, 0.0,
    1.0, 1.0,
    0.0, 1.0
];

static INDEX_DATA: [u32; 3] = [
    0, 1, 2
];

// Shader sources
static VS_SRC: &'static str =
   "#version 400\n\
    in vec3 position;\n\
    in vec2 texcoord;\n\
    in vec4 color;\n\
    out vec2 vTexcoord;\n\
    out vec4 vColor;\n\
    void main() {\n\
        vTexcoord = texcoord;\n\
        vColor = color;\n\
        gl_Position = vec4(position, 1.0);\n\
    }";

static FS_SRC: &'static str =
   "#version 400\n\
    in vec2 vTexcoord;\n\
    in vec4 vColor;\n\
    uniform sampler2D diffuseTexture;\n\
    out vec4 out_color;\n\
    void main() {\n\
        vec4 diffuse = texture2D(diffuseTexture, vTexcoord);\n\
        out_color = diffuse * vColor;\n\
    }";


fn main() {
    let mut VERTEX_COL_DATA: [GLfloat; 12] = [
        1.0, 1.0, 1.0, 1.0,
        1.0, 0.0, 1.0, 1.0,
        0.0, 1.0, 1.0, 1.0
    ];

    let mut ctx = Context::new("data/config.json");

    let t = texture::Texture::from_image("data/rust.png");

    let mut program = shader::Program::new();
    let ref vs = shader::Shader::new(shader::ShaderType::VERTEX,"data/shaders/test.vs".to_string());
    let ref fs = shader::Shader::new(shader::ShaderType::FRAGMENT,"data/shaders/test.frag".to_string());
    program.attach(vs);
    program.attach(fs);
    program.link();
    // let 

    // let vs = shader::compile_shader(VS_SRC, gl::VERTEX_SHADER);
    // let fs = shader::compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    // let program = shader::link_program(vs, fs);

    let mut m0 = mesh::Mesh::new(&VERTEX_DATA, &INDEX_DATA, Some(&VERTEX_TEX_DATA), Some(&VERTEX_COL_DATA));

    unsafe {
        program.bind();
        // gl::UseProgram(program.program_id);
        gl::BindFragDataLocation(program.program_id, 0,
            CString::new("out_color").unwrap().as_ptr());

        gl::ActiveTexture(gl::TEXTURE0);
        program.set_uniform_1i("diffuseTexture",0);
        // let diffuseTexLoc = gl::GetUniformLocation(program.program_id, CString::new("diffuseTexture").unwrap().as_ptr());
        // gl::Uniform1i(diffuseTexLoc, 0);
    }

    t.bind();

    let mut rng = rand::thread_rng();

    let mut start_time = time::now();
    let mut accum = 0.0;

    while ctx.is_running() {
        ctx.start_frame();

        let frame_time = time::now();
        let elapsed_duration = frame_time - start_time;
        start_time = frame_time;

        let mut elapsed = elapsed_duration.num_seconds() as f64;
        elapsed += (elapsed_duration.num_milliseconds() as f64) / 1_000.0;

        accum += elapsed;
        
        if accum >= 1.0 {
            accum -= 1.0;

            // try modifying tri color
            for i in 0..12 {
                if i % 4 != 0 {
                    VERTEX_COL_DATA[i] = rng.gen::<f32>();
                }
            }
            m0.update_buffer(mesh::MeshAttrib::Color, &VERTEX_COL_DATA);
        }

        m0.render();


        ctx.end_frame();
    }

    // free mem
    //TODO add memory cleaner for program and shader
    unsafe {
        // gl::DeleteProgram(program);
        // gl::DeleteShader(vs);
        // gl::DeleteShader(fs);
    }
}
