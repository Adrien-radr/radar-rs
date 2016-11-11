extern crate gl;

use std::mem;
use std::ptr;
use self::gl::types::*;

pub struct Vao {
    id: GLuint,
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.id); }
    }
}

impl Vao {
    pub fn new() -> Vao {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
            gl::BindVertexArray(id);
        }
        let vao = Vao{ id: id };
        vao
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id); }
    }

    // static fn
    pub fn unbind() {
        unsafe { gl::BindVertexArray(0); }
    }

    pub fn draw(&self, idx_count: i32) {
        // Adrien TODO - glDrawElementArrays instead
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, idx_count); }
    }
}

pub enum VboType {
    Vertex,
    Index
}

impl VboType {
    pub fn to_gl_type(&self) -> GLenum {
        match self {
            &VboType::Vertex => gl::ARRAY_BUFFER,
            &VboType::Index => gl::ELEMENT_ARRAY_BUFFER
        }
    }
}

pub struct Vbo {
    id: GLuint,
    ty: VboType
}

impl Drop for Vbo {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id); }
    }
}

impl Vbo {
    pub fn new(ty: VboType) -> Vbo {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        let vbo = Vbo { id: id, ty: ty };
        vbo
    }

    pub fn from_data<T>(data: &[T], ty: VboType) -> Vbo {
        let vbo = Vbo::new(ty);

        // get data size in bytes
        let indiv_size = mem::size_of::<T>();
        let buf_size = (indiv_size * data.len()) as GLsizeiptr;

        if buf_size > 0 {
            vbo.bind();
            unsafe {
                let dataptr = mem::transmute(&data[0]);
                gl::BufferData(vbo.ty.to_gl_type(), buf_size, dataptr, gl::STATIC_DRAW);
            }
        }
        vbo
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(self.ty.to_gl_type(), self.id); }
    }
}

pub struct Mesh {
    vao: Vao,
    vbos: [Vbo; 1], // 0: position, 1: colour
    vertex_count: i32
}

impl Mesh {
    pub fn new<T>(positions: &[T]) -> Mesh {
        let vao = Vao::new();
        let vbo = Vbo::from_data(positions, VboType::Vertex);
        let vcount = positions.len() as i32;

        let mesh = Mesh { 
            vao: vao, 
            vbos: [vbo], 
            vertex_count: vcount 
        };

        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
        }

        mesh
    }

    pub fn render(&self) {
        self.vao.draw(self.vertex_count);
    }
}