use super::material::Material;
use super::opengl::vertex_objects::Primitive;
use super::opengl::vertex_objects::VertexArrayObject;

use crate::maths::matrix::Mat4f;

pub struct Mesh {
    vao: VertexArrayObject,
    primitive: Primitive,
    material: Material,
}

impl Mesh {
    pub fn create(positions: Vec<f32>, tex_coords: Option<Vec<f32>>, primitive: Primitive, material: Material) -> Mesh {
        let mut vao = VertexArrayObject::create(positions, tex_coords);
        for instanced_buffer in material.get_instanced_buffer_locations() {
            vao.add_instanced_buffer(instanced_buffer.0, instanced_buffer.1);
        }
        Mesh {
            vao: vao,
            primitive: primitive,
            material: material,
        }
    }

    pub fn set_uniform_f32(&mut self, uniform_name: &str, value: f32) {
        self.material.set_uniform_f32(uniform_name, value);
    }

    pub fn set_instanced_data<T>(&self, buffer_index: usize, data: &Vec<T>) {
        self.vao.fill_instanced_buffer(buffer_index, data);
    }

    pub fn draw(&self, projection_view_matrix: &Mat4f, model_matrix: &Mat4f) {
        self.material.bind();
        self.material.set_transformation_matrices(projection_view_matrix, model_matrix);
        self.vao.draw(self.primitive);
    }

    pub fn draw_instanced(&self, instance_count: i32, projection_view_matrix: &Mat4f, model_matrix: &Mat4f) {
        self.material.bind();
        self.material.set_transformation_matrices(projection_view_matrix, model_matrix);
        self.vao.draw_instanced(self.primitive, instance_count);
    }
}
