extern crate glfw;
extern crate gl;

use std::sync::mpsc;
use self::glfw::Context as glfwContext;

use system::config;
use math::vec4::Vec4;

pub struct Context {
    pub glfw: glfw::Glfw,
    pub events: mpsc::Receiver<(f64, glfw::WindowEvent)>,
    
    pub window: glfw::Window,

    pub window_width : u32,
    pub window_height: u32,

    config: config::Config,

    key_state: Vec<bool>,
    prev_key_state: Vec<bool>,
    mouse_state: Vec<bool>,
    prev_mouse_state: Vec<bool>,


    clear_color: Vec4
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
        window.set_all_polling(true);

        Context::load_gl_procs(&mut window);

        unsafe{
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        
            gl::Viewport(0, 0, winw as i32, winh as i32);
        }

        let default_clear_color = Vec4::new(0.2, 0.2, 0.2, 1.0);

        Context {
            glfw: ctx,
            events: events,
            
            window: window,

            window_width: winw,
            window_height: winh,

            config: conf,
            key_state: vec![false; 1024],
            prev_key_state: vec![false; 1024],
            mouse_state: vec![false; 16],
            prev_mouse_state: vec![false; 16],

            clear_color: default_clear_color
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
                glfw::WindowEvent::MouseButton(mb, glfw::Action::Press, _) => {
                    self.mouse_state[mb as usize] = true;
                }
                glfw::WindowEvent::MouseButton(mb, glfw::Action::Release, _) => {
                    self.mouse_state[mb as usize] = false;
                }
                _ => {}
            }
        }
    }

    fn handle_events(&mut self) {
        // cpy last frame events as previous state
        self.prev_key_state.clone_from(&self.key_state);
        self.prev_mouse_state.clone_from(&self.mouse_state);

        self.glfw.poll_events();

        let mut evts: Vec<glfw::WindowEvent> = Vec::new();
        for (_, event) in glfw::flush_messages(&self.events) {
            evts.push(event);
        }
        self.handle_window_events(&evts);
    }

    pub fn is_key_hit(&self, key: glfw::Key) -> bool {
        self.key_state[key as usize] == true && self.prev_key_state[key as usize] == false
    }

    pub fn is_key_released(&self, key: glfw::Key) -> bool {
        self.key_state[key as usize] == false && self.prev_key_state[key as usize] == true
    }

    pub fn is_key_down(&self, key: glfw::Key) -> bool {
        self.key_state[key as usize] == true
    }

    pub fn is_mouse_hit(&self, mb: glfw::MouseButton) -> bool {
        self.mouse_state[mb as usize] == true && self.prev_mouse_state[mb as usize] == false
    }

    pub fn is_mouse_released(&self, mb: glfw::MouseButton) -> bool {
        self.mouse_state[mb as usize] == false && self.prev_mouse_state[mb as usize] == true
    }

    pub fn is_mouse_down(&self, mb: glfw::MouseButton) -> bool {
        self.mouse_state[mb as usize] == true
    }

    fn clear_buffers(&self) {
        unsafe {
            gl::ClearColor(self.clear_color.x, self.clear_color.y, self.clear_color.z, self.clear_color.w);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn is_running(&self) -> bool {
        !self.window.should_close()
    }
    
    pub fn start_frame(&mut self) {
        self.handle_events();

        if self.is_key_down(glfw::Key::A) {
            let newr = self.clear_color.x - 0.01;
            if newr > 0.0 {
                self.clear_color.x = newr;
            } else {
                self.clear_color.x = 0.0;
            }
        }
        if self.is_key_down(glfw::Key::D) {
            let newr = self.clear_color.x + 0.01;
            if newr < 1.0 {
                self.clear_color.x = newr;
            } else {
                self.clear_color.x = 1.0;
            }
        }

        self.clear_buffers();
    }

    pub fn end_frame(&mut self) {
        self.window.swap_buffers();
    }
}