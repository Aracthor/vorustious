use super::material::Material;
use super::super::opengl::vertex_objects::Primitive;
use super::super::opengl::vertex_objects::VertexArrayObject;

use crate::maths::matrix::Mat4f;
use crate::maths::vector::Vect4f;

pub struct Mesh {
    vao: VertexArrayObject,
    primitive: Primitive,
    material: Material,
}

impl Mesh {
    pub fn create(primitive: Primitive, material: Material, dynamic: bool) -> Mesh {
        let mut vao = VertexArrayObject::create(dynamic);
        for instanced_buffer in material.get_instanced_buffer_locations() {
            vao.add_instanced_buffer(instanced_buffer.0, instanced_buffer.1);
        }
        Mesh {
            vao: vao,
            primitive: primitive,
            material: material,
        }
    }

    pub fn set_positions_2d(&mut self, positions: &Vec<f32>) {
        let attrib_location = self.material.get_attrib_location("vert_position");
        self.vao.set_positions_2d(positions, attrib_location);
    }

    pub fn set_positions_3d(&mut self, positions: &Vec<f32>) {
        let attrib_location = self.material.get_attrib_location("vert_position");
        self.vao.set_positions_3d(positions, attrib_location);
    }

    pub fn set_colors(&mut self, colors: &Vec<f32>) {
        let attrib_location = self.material.get_attrib_location("vert_color");
        self.vao.set_colors(colors, attrib_location);
    }

    pub fn set_texture_coords(&mut self, texture_coords: &Vec<f32>) {
        let attrib_location = self.material.get_attrib_location("vert_texture_coord");
        self.vao.set_texture_coords(texture_coords, attrib_location);
    }

    pub fn set_uniform_vector(&mut self, uniform_name: &str, value: Vect4f) {
        self.material.set_uniform_vec4(uniform_name, value);
    }

    pub fn set_uniform_matrix(&mut self, uniform_name: &str, value: &Mat4f) {
        self.material.set_uniform_mat4(uniform_name, value);
    }

    pub fn set_instanced_data<T>(&self, buffer_index: usize, data: &Vec<T>) {
        self.vao.fill_instanced_buffer(buffer_index, data);
    }

    pub fn draw(&self, projection_view_matrix: &Mat4f) {
        self.material.bind();
        self.material.set_projection_view_matrix(projection_view_matrix);
        self.vao.draw(self.primitive);
    }

    pub fn draw_2d(&self, projection_matrix: &Mat4f) {
        self.material.bind();
        self.material.set_projection_matrix(projection_matrix);
        self.vao.draw(self.primitive);
    }

    pub fn draw_instanced(&self, instance_count: i32, projection_view_matrix: &Mat4f) {
        self.material.bind();
        self.material.set_projection_view_matrix(projection_view_matrix);
        self.vao.draw_instanced(self.primitive, instance_count);
    }
}
