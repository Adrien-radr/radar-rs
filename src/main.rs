extern crate glfw;
extern crate gl;

use glfw::{Action, Context, Key};
use gl::types::*;

fn load_gl_procs(window : &mut glfw::Window) {
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let win_w = 300;
    let win_h = 300;

    let (mut window, events) = glfw.create_window(win_w, win_h, "radar-rs", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    load_gl_procs(&mut window);

    unsafe{
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    
        gl::Viewport(0, 0, win_w as i32, win_h as i32);
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_events(&mut window, event);
        }

        unsafe {
            gl::ClearColor(1.0, 0.8, 0.05, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        window.swap_buffers();

    }
}

fn handle_window_events(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}