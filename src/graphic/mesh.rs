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
    pub fn create(positions: Vec<f32>, tex_coords: Option<Vec<f32>>, primitive: Primitive, material: Material, instanced: bool) -> Mesh {
        Mesh {
            vao: VertexArrayObject::create(positions, tex_coords, instanced),
            primitive: primitive,
            material: material,
        }
    }

    pub fn set_uniform_f32(&mut self, uniform_name: &str, value: f32) {
        self.material.set_uniform_f32(uniform_name, value);
    }

    pub fn draw(&self, projection_view_matrix: &Mat4f, model_matrix: &Mat4f) {
        self.material.bind();
        self.material.set_transformation_matrices(projection_view_matrix, model_matrix);
        self.vao.draw(self.primitive);
    }

    pub fn draw_instanced(&self, instance_positions: &Vec<f32>, instance_damages: &Vec<f32>, projection_view_matrix: &Mat4f, model_matrix: &Mat4f) {
        self.material.bind();
        self.material.set_transformation_matrices(projection_view_matrix, model_matrix);
        self.vao.draw_instanced(self.primitive, instance_positions, instance_damages);
    }
}
