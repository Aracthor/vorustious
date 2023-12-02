use super::opengl::shader::Shader;
use super::opengl::texture::Texture;

use crate::maths::matrix::Mat4f;
use crate::maths::vector::Vect4f;

pub struct Material {
    shader: Shader,
    uniforms_f32: Vec<(String, f32)>,
    uniforms_vect4: Vec<(String, Vect4f)>,
    textures: Vec<Texture>,
}

impl Material {
    pub fn create(vertex_file_name: &str, fragment_file_name: &str) -> Self {
        Self {
            shader: Shader::create_shader_program(vertex_file_name, fragment_file_name),
            uniforms_f32: vec![],
            uniforms_vect4: vec![],
            textures: vec![],
        }
    }

    pub fn add_texture(&mut self, texture: Texture) {
        self.textures.push(texture);
    }

    pub fn add_uniform_f32(&mut self, uniform_name: &str, value: f32) {
        self.uniforms_f32.push((String::from(uniform_name), value));
    }

    pub fn add_uniform_vect4(&mut self, uniform_name: &str, value: Vect4f) {
        self.uniforms_vect4.push((String::from(uniform_name), value));
    }

    pub fn set_transformation_matrices(&self, projection_view_matrix: &Mat4f, model_matrix: &Mat4f) {
        self.shader.set_matrix_uniform("uni_projection_view_matrix", projection_view_matrix);
        self.shader.set_matrix_uniform("uni_model_matrix", model_matrix);
    }

    pub fn bind(&self) {
        self.shader.use_program();
        for uniform in &self.uniforms_f32 {
            self.shader.set_float_uniform(&uniform.0, uniform.1);
        }
        for uniform in &self.uniforms_vect4 {
            self.shader.set_vector_uniform(&uniform.0, &uniform.1);
        }
        for texture in &self.textures {
            texture.bind();
        }
    }
}
