use super::opengl::shader::Shader;
use super::opengl::texture::Texture;

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

    pub fn set_transformation_matrices(&self, projection_view_matrix: &Mat4f, model_matrix: &Mat4f) {
        self.shader.set_matrix_uniform("uni_projection_view_matrix", projection_view_matrix);
        self.shader.set_matrix_uniform("uni_model_matrix", model_matrix);
    }

    pub fn bind(&self) {
        self.shader.use_program();
        for texture in &self.textures {
            texture.bind();
        }
    }
}
