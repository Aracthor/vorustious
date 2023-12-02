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

struct VertexBufferObject {
    id: gl::types::GLuint,
}

impl VertexBufferObject {
    pub unsafe fn create_static(data: &Vec<f32>, attrib_index: gl::types::GLuint, component_size: gl::types::GLint) -> Self {
        let mut id = 0;
        gl::GenBuffers(1, &mut id);
        let vbo = Self {id: id };
        vbo.bind();
        gl::VertexAttribPointer(attrib_index, component_size, gl::FLOAT, gl::FALSE, 0, std::ptr::null::<_>());
        gl::EnableVertexAttribArray(attrib_index);
        let size_in_bytes = (std::mem::size_of::<f32>() * data.len()).try_into().unwrap();
        gl::BufferData(gl::ARRAY_BUFFER, size_in_bytes, data.as_ptr().cast(), gl::STATIC_DRAW);
        gl_check();
        vbo.unbind();
        vbo
    }

    pub unsafe fn bind(&self) {
        gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
    }

    pub unsafe fn unbind(&self) {
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
}

impl Drop for VertexBufferObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

#[allow(dead_code)] // For buffer_objects storage, until I'm sure of their life expectancy...
pub struct VertexArrayObject {
    id: gl::types::GLuint,
    buffer_objects: Vec<VertexBufferObject>,
    element_count: i32,
}

impl VertexArrayObject {
    pub fn create(positions: Vec<f32>, texture_coords: Option<Vec<f32>>) -> VertexArrayObject {
        let mut vao = 0;
        let mut buffer_objects: Vec<VertexBufferObject> = Default::default();

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            let position_vbo = VertexBufferObject::create_static(&positions, 0, 3);
            buffer_objects.push(position_vbo);

            if texture_coords.is_some() {
                assert!(positions.len() / 3 == texture_coords.as_ref().unwrap().len() / 2);
                let texture_coords_vbo = VertexBufferObject::create_static(&texture_coords.unwrap(), 1, 2);
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
        }
    }
}
