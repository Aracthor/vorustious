extern crate gl;

pub struct VertexArrayObject {
    id: gl::types::GLuint,
    buffer_objects: Vec<gl::types::GLuint>,
    element_count: i32,
}

impl VertexArrayObject {
    pub fn create() -> VertexArrayObject {
        let mut vao = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao) };
        VertexArrayObject {
            id: vao,
            buffer_objects: vec![],
            element_count: 0,
        }
    }

    pub fn set_vertices(&mut self, vertices_data: Vec<f32>) {
        let size_in_bytes = (std::mem::size_of::<f32>() * vertices_data.len()).try_into().unwrap();
        self.element_count = (vertices_data.len() / 3).try_into().unwrap();
        unsafe {
            self.bind();
            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, size_in_bytes, vertices_data.as_ptr().cast(), gl::STATIC_DRAW);

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null::<_>());
            gl::EnableVertexAttribArray(0);

            self.buffer_objects.push(vbo);
            self.unbind();
        }
    }

    pub fn draw(&self) {
        unsafe {
            self.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, self.element_count);
            self.unbind();
        }
    }

    unsafe fn bind(&self) {
        gl::BindVertexArray(self.id);
    }

    unsafe fn unbind(&self) {
        gl::BindVertexArray(0);
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
