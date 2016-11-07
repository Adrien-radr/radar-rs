extern crate glfw;
extern crate gl;

use std::sync::mpsc;
use self::glfw::Context as glfwContext;

use system::config;

pub struct Context {
    pub glfw: glfw::Glfw,
    pub events: mpsc::Receiver<(f64, glfw::WindowEvent)>,
    
    pub window: glfw::Window,

    pub window_width : u32,
    pub window_height: u32,

    config: config::Config,

    key_state: Vec<bool>,
    prev_key_state: Vec<bool>
}

impl Context {

    fn load_gl_procs(window : &mut glfw::Window) {
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    }

    pub fn new(config_file: &str) -> Context {
        // load config file first
        let conf = config::Config::new(config_file);

        let ctx = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let winw = conf.get_u64("iWindowWidth") as u32;
        let winh = conf.get_u64("iWindowHeight") as u32;

        let (mut window, events) = ctx.create_window(winw, winh, "radar-rs", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.make_current();
        window.set_key_polling(true);

        Context::load_gl_procs(&mut window);

        unsafe{
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        
            gl::Viewport(0, 0, winw as i32, winh as i32);
        }

        Context {
            glfw: ctx,
            events: events,
            
            window: window,

            window_width: winw,
            window_height: winh,

            config: conf,

            key_state: vec![false; 256],
            prev_key_state: vec![false; 256]
        }
    }

    fn handle_window_events(&mut self, events: &[glfw::WindowEvent]) {
        for event in events {
            match *event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    self.window.set_should_close(true)
                }
                glfw::WindowEvent::Key(k, _, glfw::Action::Press, _) => {
                    self.key_state[k as usize] = true;
                }
                glfw::WindowEvent::Key(k, _, glfw::Action::Release, _) => {
                    self.key_state[k as usize] = false;
                }
                _ => {}
            }
        }
    }

    fn handle_events(&mut self) {
        // cpy last frame events as previous state
        // array_mut_ref![self.prev_key_state; 0; 256] = *array_ref![self.key_state; 0; 256];
        self.prev_key_state.clone_from(&self.key_state);

        self.glfw.poll_events();

        let mut evts: Vec<glfw::WindowEvent> = Vec::new();
        for (_, event) in glfw::flush_messages(&self.events) {
            evts.push(event);
        }
        self.handle_window_events(&evts);
    }

    pub fn run(&mut self) {
        self.handle_events();

        if self.key_state[glfw::Key::A as usize] == true && self.prev_key_state[glfw::Key::A as usize] == false {
            println!("Key A hit");
        }
        if self.key_state[glfw::Key::A as usize] == false && self.prev_key_state[glfw::Key::A as usize] == true {
            println!("Key A released");
        }
    }
}