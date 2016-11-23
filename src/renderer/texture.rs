extern crate gl;
extern crate image;

use self::gl::types::*;
use self::image::GenericImage;
use system::filesystem;
use std::path::Path;
use std::os::raw::c_void;

static VALID_IMG_EXT: [&'static str; 5] = [
    "png", "jpeg", "jpg", "gif", "bmp"
];

pub enum TextureFmt {
    R8U,
    RG8U,
    RGB8U,
    RGBA8U
}

impl TextureFmt {
    pub fn gl_format(&self) -> GLenum {
        match self {
            &TextureFmt::R8U => gl::RED,
            &TextureFmt::RG8U => gl::RG,
            &TextureFmt::RGB8U => gl::RGB,
            &TextureFmt::RGBA8U => gl::RGBA,
        }
    }

    pub fn gl_type(&self) -> GLenum {
        match self {
            &TextureFmt::R8U => gl::UNSIGNED_BYTE,
            &TextureFmt::RG8U => gl::UNSIGNED_BYTE,
            &TextureFmt::RGB8U => gl::UNSIGNED_BYTE,
            &TextureFmt::RGBA8U => gl::UNSIGNED_BYTE,
        }
    }

    // pub fn gl_component_count(&self) -> usize {
        // match self {
            // &R8U => 1,
            // &RG8U => 2,
            // &RGB8U => 3,
            // &RGBA8U => 4,
        // }
    // }

    pub fn gl_bpp(&self) -> usize {
        match self {
            &TextureFmt::R8U => 1,
            &TextureFmt::RG8U => 2,
            &TextureFmt::RGB8U => 3,
            &TextureFmt::RGBA8U => 4,
        }
    }
}

fn image_to_gl_fmt(ty: image::ColorType) -> GLenum {
    use self::image::ColorType::*;

    match ty {
        Gray(_) => gl::RED,
        RGB(_) => gl::RGB,
        GrayA(_) => gl::RG,
        RGBA(_) => gl::RGBA,
        _ => gl::NONE
    }
}

fn image_to_radar_fmt(ty: image::ColorType) -> TextureFmt {
    use self::image::ColorType::*;

    match ty {
        Gray(_) => TextureFmt::R8U,
        RGB(_) => TextureFmt::RGB8U,
        GrayA(_) => TextureFmt::RG8U,
        RGBA(_) => TextureFmt::RGBA8U,
        _ => TextureFmt::R8U
    }
}

fn get_pixel_ptr(img: &image::DynamicImage) -> *const u8 {
    use self::image::ColorType::*;
    let ty = img.color();

    match ty {
        Gray(_) => (*img.as_luma8().unwrap()).as_ptr(),
        RGB(_) => (*img.as_rgb8().unwrap()).as_ptr(),
        GrayA(_) => (*img.as_luma_alpha8().unwrap()).as_ptr(),
        RGBA(_) => (*img.as_rgba8().unwrap()).as_ptr(),
        _ => {panic!("Unsupported pixel format.");}
    }
}

pub struct Texture {
    pub id: GLuint,
    pub size: (u32, u32),
    pub fmt: TextureFmt
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id); }
    }
}

impl Texture {
    pub fn from_image(path_str: &str) -> Texture {
        let mut id = 0u32;

        let path = Path::new(path_str);

        //1. check extension
        if !filesystem::check_extension(path, &VALID_IMG_EXT) {
            panic!("Invalid image file {}.", path.display());
        }

        //2. load image
        let img = image::open(&path).unwrap();

        let fmt = image_to_gl_fmt(img.color());
        if fmt == gl::NONE {
            panic!("Unsupported texture format for {}.", path.display());
        }

        let dims = img.dimensions();
        let w = dims.0 as i32;
        let h = dims.1 as i32;
        let bytes = get_pixel_ptr(&img);

        //3. convert to gl texture
        unsafe {
            let mut palign: GLint = 1;
            gl::GetIntegerv(gl::UNPACK_ALIGNMENT, &mut palign);
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_ANISOTROPY_EXT, gl::REPEAT);
            // ADRIEN TODO - anisotropic level from config file : needs global access to config file somehow
            gl::TexImage2D(gl::TEXTURE_2D, 0, fmt as i32, w, h, 0, fmt, gl::UNSIGNED_BYTE, bytes as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::PixelStorei(gl::UNPACK_ALIGNMENT, palign);
        }

        Texture { 
            id: id,
            size: (dims.0, dims.1),
            fmt: image_to_radar_fmt(img.color())
        }
    }

    // ADRIEN TODO - more generic function to allow textures of any type (f32, int, etc)
    pub fn from_empty(size: (u32, u32), fmt: TextureFmt) -> Texture {
        let mut id = 0u32;
        let gl_bpp = fmt.gl_bpp() as usize;
        let gl_fmt = fmt.gl_format();
        let gl_type = fmt.gl_type();
        let w = size.0 as i32;
        let h = size.1 as i32;

        let empty_arr = vec![0; gl_bpp * size.0 as usize * size.1 as usize];

        unsafe {
            let mut palign: GLint = 1;
            gl::GetIntegerv(gl::UNPACK_ALIGNMENT, &mut palign);
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl_fmt as i32, w, h, 0, gl_fmt, gl_type, empty_arr.as_ptr() as *const c_void);

            gl::PixelStorei(gl::UNPACK_ALIGNMENT, palign);
        }

        Texture { 
            id: id,
            size: size,
            fmt: fmt
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.id); }
    }
}