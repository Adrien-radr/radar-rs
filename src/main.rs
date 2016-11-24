mod system;
mod math;
mod renderer;
mod canvas;

use renderer::context::*;
use renderer::mesh;
use renderer::shader::{Program, Shader, ShaderType};
use renderer::texture;
use math::mat4::*;
use math::transform;
use math::vec3::*;
use canvas::Canvas;

extern crate rand;
extern crate time;
extern crate gl;

extern crate specs;
use specs::Join;

use gl::types::*;
use std::mem;
use std::ptr;
use std::ffi::CString;
use rand::Rng;


// Vertex data
static VERTEX_DATA: [GLfloat; 9] = [
     200.0, 100.5, 0.11,
     100.5, 200.5, 0.11,
     300.5, 200.5, 0.11
];

static VERTEX_TEX_DATA: [GLfloat; 6] = [
    0.5, 0.0,
    0.0, 1.0,
    1.0, 1.0
];

static INDEX_DATA: [u32; 3] = [
    0, 1, 2
];

#[derive(Clone, Debug)]
struct CompPos(f32,f32);
impl specs::Component for CompPos{
    type Storage = specs::VecStorage<CompPos>;
}
#[derive(Clone, Debug)]
struct CompVel(f32,f32);
impl specs::Component for CompVel{
    type Storage = specs::VecStorage<CompVel>;
}
struct CompMesh(mesh::Mesh);
impl specs::Component for CompMesh{
    type Storage = specs::VecStorage<CompMesh>;     
}

fn main() {
    let mut VERTEX_COL_DATA: [GLfloat; 12] = [
        1.0, 1.0, 1.0, 1.0,
        1.0, 0.0, 1.0, 1.0,
        0.0, 1.0, 1.0, 1.0
    ];

    let mut ctx = Context::new("data/config.json");

    let t = texture::Texture::from_image("data/rust.png");


    let mut program = Program::new();
    let vs = Shader::new(ShaderType::VERTEX,"data/shaders/test.vs".to_string());
    let fs = Shader::new(ShaderType::FRAGMENT,"data/shaders/test.frag".to_string());
    program.attach(&vs);
    program.attach(&fs);
    program.link();
    program.register_uniform("ProjMatrix");
    program.register_uniform("ModelMatrix");
    program.register_uniform("diffuseTexture");

    let mut m0 = mesh::Mesh::new(&VERTEX_DATA, &INDEX_DATA, Some(&VERTEX_TEX_DATA), Some(&VERTEX_COL_DATA));

    program.set_uniform_matrix4fv("ProjMatrix", &ctx.proj_matrix_2d);
    program.set_uniform_matrix4fv("ModelMatrix", &Mat4::identity());
    program.set_uniform_1i("diffuseTexture", 0);

    let mut canvas_program = Program::new();
    let canvas_vs = Shader::new(ShaderType::VERTEX, "data/shaders/canvas.vs".to_string());
    let canvas_fs = Shader::new(ShaderType::FRAGMENT, "data/shaders/canvas.frag".to_string());
    canvas_program.attach(&canvas_vs);
    canvas_program.attach(&canvas_fs);
    canvas_program.link();
    canvas_program.register_uniform("ProjMatrix");
    canvas_program.register_uniform("ModelMatrix");
    canvas_program.register_uniform("backColor");
    canvas_program.register_uniform("diffuseTexture");
    canvas_program.register_uniform("canvasPosition");
    canvas_program.register_uniform("canvasSize");

    canvas_program.set_uniform_matrix4fv("ProjMatrix", &ctx.proj_matrix_2d);
    canvas_program.set_uniform_1i("diffuseTexture", 0);

    let mut canvas1 = Canvas::new((400, 200), (200, 100), &canvas_program);

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

        //now we update the systems
        program.bind();
        t.bind();
        m0.render();

        canvas_program.bind();
        canvas1.update(elapsed);
        canvas1.render();


        ctx.end_frame();
    }
}
