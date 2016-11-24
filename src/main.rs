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
extern crate glfw;

use gl::types::*;
use std::mem;
use std::ptr;
use std::ffi::CString;
use rand::Rng;
// use glfw::

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

fn main() {
    let mut VERTEX_COL_DATA: [GLfloat; 12] = [
        1.0, 1.0, 1.0, 1.0,
        1.0, 0.0, 1.0, 1.0,
        0.0, 1.0, 1.0, 1.0
    ];

    let mut ctx = Context::new("data/config.json");

    let t = texture::Texture::from_image("data/rust.png");

    let mut m0 = mesh::Mesh::new(&VERTEX_DATA, &INDEX_DATA, Some(&VERTEX_TEX_DATA), Some(&VERTEX_COL_DATA));

    let program_shaders = [Some("data/shaders/test.vs"), Some("data/shaders/test.frag"), None, None, None];
    let program_uniforms = ["ProjMatrix", "ModelMatrix", "diffuseTexture"];

    let canvas_program_shaders = [Some("data/shaders/canvas.vs"), Some("data/shaders/canvas.frag"), None, None, None];
    let canvas_program_uniforms = ["ProjMatrix", "ModelMatrix", "diffuseTexture", "backColor", "canvasPosition", "canvasSize"];

    let prog1 = ctx.build_shader_program("testProgram", &program_shaders, &program_uniforms);
    let prog2 = ctx.build_shader_program("canvasProgram", &canvas_program_shaders, &canvas_program_uniforms);

    {
        let p = prog1.borrow_mut();
        p.bind();
        p.set_uniform_matrix4fv("ProjMatrix", &ctx.proj_matrix_2d);
        p.set_uniform_matrix4fv("ModelMatrix", &Mat4::identity());
        p.set_uniform_1i("diffuseTexture", 0);
    }

    {
        let p = prog2.borrow_mut();
        p.bind();
        p.set_uniform_matrix4fv("ProjMatrix", &ctx.proj_matrix_2d);
        p.set_uniform_1i("diffuseTexture", 0);
    }

    let mut canvas1 = Canvas::new((000, 000), (1200, 600), prog2);
    // reload_shaders(&ctx);

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

        // if ctx.is_key_hit(glfw::Key::R) && ctx.is_key_down(glfw::Key::LeftControl) {
            // reload_shaders(&ctx);
        // }

        prog1.borrow().bind();
        t.bind();
        m0.render();

        prog2.borrow().bind();
        canvas1.update(elapsed);
        canvas1.render();

        // println!("{}, {}", elapsed, 1.0/elapsed);


        ctx.end_frame();
    }
}
