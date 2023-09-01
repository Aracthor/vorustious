use super::gl_check::gl_check;

#[derive(Clone, Copy)]
pub enum Primitive {
    Lines,
    Triangles,
}

impl Primitive {
    pub fn to_gl_primitive(&self) -> gl::types::GLenum {
        match self {
            Primitive::Lines => gl::LINES,
            Primitive::Triangles => gl::TRIANGLES,
        }
    }
}

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
        let mut buffer_objects: Vec<gl::types::GLuint> = Default::default();

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            let positions_size_in_bytes = (std::mem::size_of::<f32>() * positions.len()).try_into().unwrap();
            gl::GenBuffers(1, &mut position_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, position_vbo);
            gl::BufferData(gl::ARRAY_BUFFER, positions_size_in_bytes, positions.as_ptr().cast(), gl::STATIC_DRAW);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null::<_>());
            gl::EnableVertexAttribArray(0);
            gl_check();
            buffer_objects.push(position_vbo);

            if !texture_coords.is_empty() {
                let texture_coords_size_in_bytes = (std::mem::size_of::<f32>() * texture_coords.len()).try_into().unwrap();
                assert!(positions_size_in_bytes / 3 * 2 == texture_coords_size_in_bytes);
                gl::GenBuffers(1, &mut texture_coords_vbo);
                gl::BindBuffer(gl::ARRAY_BUFFER, texture_coords_vbo);
                gl::BufferData(gl::ARRAY_BUFFER, texture_coords_size_in_bytes, texture_coords.as_ptr().cast(), gl::STATIC_DRAW);
                gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null::<_>());
                gl::EnableVertexAttribArray(1);
                gl_check();
                buffer_objects.push(texture_coords_vbo);
            }

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        VertexArrayObject {
            id: vao,
            buffer_objects: buffer_objects,
            element_count: (positions.len() / 3).try_into().unwrap(),
        }
    }

    pub fn draw(&self, primitive: Primitive) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::DrawArrays(primitive.to_gl_primitive(), 0, self.element_count);
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
