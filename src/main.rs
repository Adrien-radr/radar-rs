extern crate glfw;
extern crate gl;

// use glfw::{Action, Context, Key};
use gl::types::*;
use glfw::Context as glfwContext;

mod math;
mod system;
mod context;

use math::vec3::Vec3;
use context::Context;

fn main() {
    let mut ctx = Context::new("data/config.json");
    let mut bck_col = Vec3::new(1.0, 0.8, 0.05);

    while !ctx.window.should_close() {
        ctx.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&ctx.events) {
            handle_window_events(&mut ctx.window, event);
        }

        unsafe {
            gl::ClearColor(bck_col.x, bck_col.y, bck_col.z, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // WTF RUST ??? NO MAX FOR F32 !?!??!
        if ctx.window.get_key(glfw::Key::A) == glfw::Action::Press {
            let newr = bck_col.x - 0.01;
            if newr > 0.0 {
                bck_col.x = newr;
            } else {
                bck_col.x = 0.0;
            }
        }
        if ctx.window.get_key(glfw::Key::D) == glfw::Action::Press {
            let newr = bck_col.x + 0.01;
            if newr < 1.0 {
                bck_col.x = newr;
            } else {
                bck_col.x = 1.0;
            }
        }
        ctx.window.swap_buffers();

    }
}

fn handle_window_events(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}