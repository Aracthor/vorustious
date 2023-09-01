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
    pub fn create(positions: Vec<f32>, tex_coords: Vec<f32>, primitive: Primitive, material: Material) -> Mesh {
        Mesh {
            vao: VertexArrayObject::create(positions, tex_coords),
            primitive: primitive,
            material: material,
        }
    }

    pub fn draw(&self, projection_view_matrix: &Mat4f, model_matrix: &Mat4f) {
        self.material.bind();
        self.material.set_transformation_matrices(projection_view_matrix, model_matrix);
        self.vao.draw(self.primitive);
    }
}
