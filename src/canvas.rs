use renderer::mesh::*;
use renderer::texture::*;
use renderer::shader::*;
use math::vec3::*;
use math::vec4::*;
use math::mat4::*;
use math::transform;

use renderer::context::Context;

static WIDGET_VERT_POS : [f32; 12] = [
    0.0, 0.0, 1.0,
    0.0, 1.0, 1.0,
    1.0, 1.0, 1.0,
    1.0, 0.0, 1.0
];

static WIDGET_VERT_TEX : [f32; 8] = [
    0.0, 0.0,
    0.0, 1.0,
    1.0, 1.0,
    1.0, 0.0
];

static WIDGET_INDEX : [u32; 6] = [
    0, 1, 2, 0, 2, 3
];

pub struct Widget<'a> {
    position: (u32, u32),
    size: (u32, u32),
    mesh: Mesh,
    shader_handle: usize,

    backColor: Vec4,

    model_matrix: Mat4,
    pos_changed: bool,

    ctx: &'a Context
}

impl<'a> Widget<'a> {
    pub fn new(ctx: &'a mut Context, pos: (u32, u32), size: (u32, u32), shader_h: usize) -> Widget {
        let m = Mesh::new(&WIDGET_VERT_POS, &WIDGET_INDEX, Some(&WIDGET_VERT_TEX), None);

        Widget {
            position: pos,
            size: size,
            mesh: m,
            shader_handle: shader_h,
            backColor: Vec4::new(0.9,0.9,0.9,1.0),
            model_matrix: Mat4::identity(),
            pos_changed: true,
            ctx: ctx
        }
    }

    fn update_model_matrix(&mut self) {
        if self.pos_changed {
            self.pos_changed = false;
            self.model_matrix = transform::scale(Vec3::new(self.size.0 as f32, self.size.1 as f32, 1.0));
            self.model_matrix *= transform::translation(Vec3::new(self.position.0 as f32, self.position.1 as f32, 0.0));
            {
                let p = self.ctx.borrow_shader(self.shader_handle);
                p.bind();
                p.set_uniform_matrix4fv("ModelMatrix", &self.model_matrix);
            }
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.update_model_matrix();
    }

    pub fn render(&self) {
        {
            let p = self.ctx.borrow_shader(self.shader_handle);
            p.bind();
            p.set_uniform_4fv("backColor", &self.backColor);
            p.set_uniform_2fv("canvasPosition", (self.position.0 as f32, self.position.1 as f32));
            p.set_uniform_2fv("canvasSize", (self.size.0 as f32, self.size.1 as f32));
        }
        self.mesh.render();
    }
}

pub struct Canvas<'a> {
    texture: Texture,
    widget: Widget<'a>
}

impl<'a> Canvas<'a> {
    pub fn new(ctx: &'a mut Context, pos: (u32, u32), size: (u32, u32), shader_program_handle: usize) -> Canvas {
        let w = Widget::new(ctx, pos, size, shader_program_handle);
        let t = Texture::from_empty(size, TextureFmt::RGB8U);

        Canvas {
            texture: t,
            widget: w
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.widget.update(dt);
    }

    pub fn render(&self) {
        self.texture.bind();
        self.widget.render();
    }
}