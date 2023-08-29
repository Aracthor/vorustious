use super::material::Material;
use super::vertex_objects::VertexArrayObject;

use crate::maths::matrix::Mat4f;

pub struct Mesh {
    vao: VertexArrayObject,
    material: Material,
}

impl Mesh {
    pub fn create(positions: Vec<f32>, tex_coords: Vec<f32>, material: Material) -> Mesh {
        Mesh {
            vao: VertexArrayObject::create(positions, tex_coords),
            material: material,
        }
    }

    pub fn draw(&self, perspective_matrix: &Mat4f) {
        self.material.bind();
        self.material.set_transformation_matrices(perspective_matrix);
        self.vao.draw();
    }
}
