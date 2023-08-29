use super::shader::Shader;
use super::texture::Texture;

use crate::maths::matrix::Mat4f;

pub struct Material {
    shader: Shader,
    textures: Vec<Texture>,
}

impl Material {
    pub fn create(vertex_file_name: &str, fragment_file_name: &str) -> Material {
        Material {
            shader: Shader::create_shader_program(vertex_file_name, fragment_file_name),
            textures: vec![],
        }
    }

    pub fn add_texture(&mut self, texture: Texture) {
        self.textures.push(texture);
    }

    pub fn set_transformation_matrices(&self, perspective_matrix: &Mat4f, view_matrix: &Mat4f) {
        self.shader.set_matrix_uniform("uni_projection_matrix", perspective_matrix);
        self.shader.set_matrix_uniform("uni_view_matrix", view_matrix);
    }

    pub fn bind(&self) {
        self.shader.use_program();
        for texture in &self.textures {
            texture.bind();
        }
    }
}
