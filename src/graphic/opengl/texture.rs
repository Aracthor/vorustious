use super::gl_check::gl_check;
use super::super::core::color::Color;

pub struct Texture {
    id: gl::types::GLuint,
}

impl Texture {
    pub fn create(width: i32, height: i32, pixels: Vec<Color>) -> Texture {
        assert!(width * height == pixels.len().try_into().unwrap());
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA.try_into().unwrap(), width, height, 0, gl::RGBA, gl::UNSIGNED_BYTE, pixels.as_ptr().cast());
            gl_check();

            // Set the filtering mode.
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST.try_into().unwrap());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST.try_into().unwrap());
            gl_check();

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Texture { id: id }
    }

    pub fn bind(&self, texture_index: usize) {
        let texture_id = match texture_index {
            0 => gl::TEXTURE0,
            1 => gl::TEXTURE1,
            2 => gl::TEXTURE2,
            3 => gl::TEXTURE3,
            4 => gl::TEXTURE4,
            _ => unimplemented!(),
        };
        unsafe {
            gl::ActiveTexture(texture_id);
            gl::BindTexture(gl::TEXTURE_2D, self.id)
        };
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id) };
    }
}