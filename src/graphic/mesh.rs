use super::shader::Shader;
use super::vertex_objects::VertexArrayObject;

pub struct Mesh {
    vao: VertexArrayObject,
    material: Shader,
}

impl Mesh {
    pub fn create(vertices_data: Vec<f32>, material: Shader) -> Mesh {
        Mesh {
            vao: VertexArrayObject::create(vertices_data),
            material: material,
        }
    }

    pub fn draw(&self) {
        self.material.use_program();
        self.vao.draw();
    }
}
