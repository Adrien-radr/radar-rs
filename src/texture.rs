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

fn to_glfmt(ty: image::ColorType) -> GLenum {
    use self::image::ColorType::*;

    match ty {
        Gray(u8) => gl::RED,
        RGB(u8) => gl::RGB,
        GrayA(u8) => gl::RG,
        RGBA(u8) => gl::RGBA,
        _ => gl::NONE
    }
}

fn get_pixel_ptr(img: &image::DynamicImage) -> *const u8 {
    use self::image::ColorType::*;
    let ty = img.color();

    match ty {
        Gray(u8) => (*img.as_luma8().unwrap()).as_ptr(),
        RGB(u8) => (*img.as_rgb8().unwrap()).as_ptr(),
        GrayA(u8) => (*img.as_luma_alpha8().unwrap()).as_ptr(),
        RGBA(u8) => (*img.as_rgba8().unwrap()).as_ptr(),
        _ => {panic!("Unsupported pixel format.");}
    }
}

pub struct Texture {
    pub id: GLuint,
    pub size: (u32, u32)
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

        let fmt = to_glfmt(img.color());
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

        let tex = Texture { 
            id: id,
            size: (dims.0, dims.1) 
        };
        tex
    }

    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.id); }
    }
}