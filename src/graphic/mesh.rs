use super::material::Material;
use super::vertex_objects::VertexArrayObject;

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

    pub fn draw(&self) {
        self.material.bind();
        self.vao.draw();
    }
}
