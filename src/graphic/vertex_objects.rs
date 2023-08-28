extern crate gl;

pub struct VertexArrayObject {
    id: gl::types::GLuint,
    buffer_objects: Vec<gl::types::GLuint>,
    element_count: i32,
}

impl VertexArrayObject {
    pub fn create(vertices_data: Vec<f32>) -> VertexArrayObject {
        let mut vao = 0;
        let mut vbo = 0;
        let size_in_bytes = (std::mem::size_of::<f32>() * vertices_data.len()).try_into().unwrap();

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(gl::ARRAY_BUFFER, size_in_bytes, vertices_data.as_ptr().cast(), gl::STATIC_DRAW);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null::<_>());
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        VertexArrayObject {
            id: vao,
            buffer_objects: vec![vbo],
            element_count: (vertices_data.len() / 3).try_into().unwrap(),
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
