use super::gl_check::gl_check;

pub struct Texture {
    id: gl::types::GLuint,
}

impl Texture {
    pub fn create(width: i32, height: i32, bytes: Vec<u8>) -> Texture {
        assert!(width * height == (bytes.len() / 4).try_into().unwrap());

        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA.try_into().unwrap(), width, height, 0, gl::RGBA, gl::UNSIGNED_BYTE, bytes.as_ptr().cast());
            gl_check();

            // Set the filtering mode.
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST.try_into().unwrap());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST.try_into().unwrap());
            gl_check();

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Texture { id: id }
    }

    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.id) };
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id) };
    }
}