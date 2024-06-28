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
    pub unsafe fn new(attrib_index: gl::types::GLuint, component_size: gl::types::GLint) -> Self {
        let mut id = 0;
        gl::GenBuffers(1, &mut id);
        let vbo = Self {
            id: id,
        };
        vbo.bind();
        gl::VertexAttribPointer(attrib_index, component_size, gl::FLOAT, gl::FALSE, 0, std::ptr::null::<_>());
        gl::EnableVertexAttribArray(attrib_index);
        gl_check();
        vbo.unbind();
        vbo
    }

    pub unsafe fn set_data<T>(&self, data: &Vec<T>, usage: gl::types::GLenum) {
        self.bind();
        let size_in_bytes = (std::mem::size_of::<T>() * data.len()).try_into().unwrap();
        gl::BufferData(gl::ARRAY_BUFFER, size_in_bytes, data.as_ptr().cast(), usage);
        self.unbind();
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

pub struct VertexArrayObject {
    id: gl::types::GLuint,
    position_buffer: Option<VertexBufferObject>,
    texture_coords_buffer: Option<VertexBufferObject>,
    instance_buffer_objects: Vec<VertexBufferObject>,
    element_count: i32,
}

impl VertexArrayObject {
    pub fn create() -> VertexArrayObject {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        VertexArrayObject {
            id: vao,
            position_buffer: None,
            texture_coords_buffer: None,
            instance_buffer_objects: Default::default(),
            element_count: 0,
        }
    }

    pub fn set_positions(&mut self, positions: Vec<f32>) {
        assert!(self.position_buffer.is_none());
        assert!(positions.len() % 3 == 0);
        unsafe {
            gl::BindVertexArray(self.id);
            self.position_buffer = Some(VertexBufferObject::new(0, 3));
            self.position_buffer.as_mut().unwrap().set_data(&positions, gl::STATIC_DRAW);
            gl::BindVertexArray(0);
        }
        self.element_count = (positions.len() / 3 as usize).try_into().unwrap();
    }

    pub fn set_texture_coords(&mut self, texture_coords: Vec<f32>) {
        assert!(self.texture_coords_buffer.is_none());
        assert!(texture_coords.len() % 2 == 0);
        unsafe {
            gl::BindVertexArray(self.id);
            self.texture_coords_buffer = Some(VertexBufferObject::new(1, 2));
            self.texture_coords_buffer.as_mut().unwrap().set_data(&texture_coords, gl::STATIC_DRAW);
            gl::BindVertexArray(0);
        }
    }

    pub fn add_instanced_buffer(&mut self, attrib_index: u32, component_size: i32) {
        unsafe {
            gl::BindVertexArray(self.id);
            self.instance_buffer_objects.push(VertexBufferObject::new(attrib_index, component_size));
            gl_check();
            gl::VertexAttribDivisor(attrib_index, 1);
            gl::BindVertexArray(0);
        }
    }

    pub fn fill_instanced_buffer<T>(&self, buffer_index: usize, data: &Vec<T>) {
        unsafe {
            self.instance_buffer_objects[buffer_index].set_data(data, gl::DYNAMIC_DRAW);
        }
    }

    pub fn draw(&self, primitive: Primitive) {
        assert!(!self.instanced());
        unsafe {
            gl::BindVertexArray(self.id);
            gl::DrawArrays(primitive.to_gl_primitive(), 0, self.element_count);
            gl::BindVertexArray(0);
        }
    }

    pub fn draw_instanced(&self, primitive: Primitive, instance_count: i32) {
        assert!(self.instanced());
        unsafe {
            gl::BindVertexArray(self.id);
            gl::DrawArraysInstanced(primitive.to_gl_primitive(), 0, self.element_count, instance_count);
            gl::BindVertexArray(0);
        }
    }

    fn instanced(&self) -> bool {
        !self.instance_buffer_objects.is_empty()
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
