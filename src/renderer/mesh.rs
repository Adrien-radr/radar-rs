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
        unsafe { gl::DrawElements(gl::TRIANGLES, idx_count, gl::UNSIGNED_INT, ptr::null()); }
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
    ty: VboType,
    data_count: usize
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
        let vbo = Vbo { id: id, ty: ty, data_count: 0 };
        vbo
    }

    pub fn from_data<T>(data: &[T], ty: VboType) -> Vbo {
        let mut vbo = Vbo::new(ty);
        vbo.data_count = data.len();

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

    pub fn update<T>(&mut self, data: &[T]) {
        assert_eq!(data.len(), self.data_count);

        self.bind();
        let buf_size = (mem::size_of::<T>() * self.data_count) as GLsizeiptr;
        unsafe {
            let dataptr = mem::transmute(&data[0]);
            gl::BufferSubData(self.ty.to_gl_type(), 0, buf_size, dataptr);
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(self.ty.to_gl_type(), self.id); }
    }
}

#[derive(Clone)]
pub enum MeshAttrib {
    Position = 0,
    Texcoord = 1,
    Color = 2
}

impl MeshAttrib {
    pub fn get_component_count(&self) -> GLint {
        match self {
            &MeshAttrib::Position => 3,
            &MeshAttrib::Texcoord => 2,
            &MeshAttrib::Color => 4
        }
    }
}

pub struct Mesh {
    vao: Vao,
    vbos: [Option<Vbo>; 3], // 0: position, 1: texcoord, 2: color
    ibo: Vbo,
    vertex_count: i32,
    index_count: i32
}

impl Mesh {
    fn make_attrib_vbo<T>(data: Option<&[T]>, ma: MeshAttrib) -> Option<Vbo> { 
        let comp_cnt = ma.get_component_count();
        let ma_u32 = ma as u32;
        
        match data {
            Some(arr) => {
                let v = Vbo::from_data(arr, VboType::Vertex);

                unsafe {
                    gl::EnableVertexAttribArray(ma_u32);
                    gl::VertexAttribPointer(ma_u32, comp_cnt, gl::FLOAT, gl::FALSE, 0, ptr::null());
                } 
                Some(v)
            },
            None => {
                unsafe {
                    gl::DisableVertexAttribArray(ma_u32);
                }
                None
            }
        }
    }

    pub fn new<T>(positions: &[T], indices: &[u32], texcoords: Option<&[T]>, colors: Option<&[T]>) -> Mesh {
        let vao = Vao::new();
        let vcount = positions.len() as i32 / 3;
        let icount = indices.len() as i32;

        // position
        let vbo_pos = Mesh::make_attrib_vbo(Some(positions), MeshAttrib::Position);

        // indices
        let ibo = Vbo::from_data(indices, VboType::Index);

        // texcord
        let vbo_tex = Mesh::make_attrib_vbo(texcoords, MeshAttrib::Texcoord);

        // color
        let vbo_col = Mesh::make_attrib_vbo(colors, MeshAttrib::Color);

        let mesh = Mesh { 
            vao: vao, 
            vbos: [vbo_pos, vbo_tex, vbo_col], 
            ibo: ibo,
            vertex_count: vcount,
            index_count: icount
        };

        mesh
    }

    pub fn update_buffer<T>(&mut self, attrib_idx: MeshAttrib, data: &[T]) {
        let components = attrib_idx.get_component_count();
        let idx = attrib_idx.clone() as usize;

        self.vao.bind();
        
        // either update or create data if nonexistent
        match self.vbos[idx] {
            Some(ref mut vb) => vb.update(data),
            None => {
                self.vbos[idx] = Mesh::make_attrib_vbo(Some(data), attrib_idx);
            }
        }

    }

    pub fn render(&self) {
        self.vao.bind();
        self.vao.draw(self.index_count);
    }
}