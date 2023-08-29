use super::gl_check::gl_check;

pub struct VertexArrayObject {
    id: gl::types::GLuint,
    buffer_objects: Vec<gl::types::GLuint>,
    element_count: i32,
}

impl VertexArrayObject {
    pub fn create(positions: Vec<f32>, texture_coords: Vec<f32>) -> VertexArrayObject {
        let mut vao = 0;
        let mut position_vbo = 0;
        let mut texture_coords_vbo = 0;
        let positions_size_in_bytes = (std::mem::size_of::<f32>() * positions.len()).try_into().unwrap();
        let texture_coords_size_in_bytes = (std::mem::size_of::<f32>() * texture_coords.len()).try_into().unwrap();
        assert!(positions_size_in_bytes / 3 * 2 == texture_coords_size_in_bytes);

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut position_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, position_vbo);
            gl::BufferData(gl::ARRAY_BUFFER, positions_size_in_bytes, positions.as_ptr().cast(), gl::STATIC_DRAW);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null::<_>());
            gl::EnableVertexAttribArray(0);
            gl_check();

            gl::GenBuffers(1, &mut texture_coords_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, texture_coords_vbo);
            gl::BufferData(gl::ARRAY_BUFFER, texture_coords_size_in_bytes, texture_coords.as_ptr().cast(), gl::STATIC_DRAW);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null::<_>());
            gl::EnableVertexAttribArray(1);
            gl_check();

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        VertexArrayObject {
            id: vao,
            buffer_objects: vec![position_vbo, texture_coords_vbo],
            element_count: (positions.len() / 3).try_into().unwrap(),
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::DrawArrays(gl::TRIANGLES, 0, self.element_count);
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
            for vbo in &self.buffer_objects {
                gl::DeleteBuffers(1, vbo);
            }
        }
    }
}
