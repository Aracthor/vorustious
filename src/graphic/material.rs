use std::collections::HashMap;

use super::opengl::shader::Shader;
use super::opengl::texture::Texture;

use crate::maths::matrix::Mat4f;
use crate::maths::vector::Vect4f;

pub struct Material {
    shader: Shader,
    uniforms_f32: HashMap<String, f32>,
    uniforms_vect4: HashMap<String, Vect4f>,
    uniforms_mat4: HashMap<String, Mat4f>,
    textures: HashMap<String, Texture>,
    instance_data_buffers: Vec<(String, i32)>,
}

impl Material {
    pub fn create(vertex_file_name: &str, fragment_file_name: &str) -> Self {
        Self {
            shader: Shader::create_shader_program(vertex_file_name, fragment_file_name),
            uniforms_f32: Default::default(),
            uniforms_vect4: Default::default(),
            uniforms_mat4: Default::default(),
            textures: Default::default(),
            instance_data_buffers: Default::default(),
        }
    }

    pub fn add_texture(&mut self, name: &str, texture: Texture) {
        self.textures.insert(String::from(name), texture);
    }

    pub fn add_instance_data_buffer(&mut self, name: &str, component_size: i32) {
        self.instance_data_buffers.push((String::from(name), component_size));
    }

    pub fn add_uniform_f32(&mut self, uniform_name: &str, value: f32) {
        self.uniforms_f32.insert(String::from(uniform_name), value);
    }

    pub fn add_uniform_vect4(&mut self, uniform_name: &str, value: Vect4f) {
        self.uniforms_vect4.insert(String::from(uniform_name), value);
    }

    pub fn add_uniform_mat4(&mut self, uniform_name: &str, value: Mat4f) {
        self.uniforms_mat4.insert(String::from(uniform_name), value);
    }

    pub fn set_uniform_f32(&mut self, uniform_name: &str, value: f32) {
        assert!(self.uniforms_f32.contains_key(uniform_name));
        *self.uniforms_f32.get_mut(uniform_name).unwrap() = value;
    }

    pub fn set_uniform_mat4(&mut self, uniform_name: &str, value: &Mat4f) {
        assert!(self.uniforms_mat4.contains_key(uniform_name));
        *self.uniforms_mat4.get_mut(uniform_name).unwrap() = value.clone();
    }

    pub fn set_projection_view_matrix(&self, projection_view_matrix: &Mat4f) {
        self.shader.set_matrix_uniform("uni_projection_view_matrix", projection_view_matrix);
    }

    pub fn set_projection_matrix(&self, projection_matrix: &Mat4f) {
        self.shader.set_matrix_uniform("uni_projection_matrix", projection_matrix);
    }

    pub fn get_instanced_buffer_locations(&self) -> Vec<(u32, i32)> {
        self.instance_data_buffers.iter().map(|buffer| (self.shader.get_attrib_location(&buffer.0), buffer.1)).collect()
    }

    pub fn bind(&self) {
        self.shader.use_program();
        for uniform in &self.uniforms_f32 {
            self.shader.set_float_uniform(uniform.0, *uniform.1);
        }
        for uniform in &self.uniforms_vect4 {
            self.shader.set_vector_uniform(uniform.0, uniform.1);
        }
        for uniform in &self.uniforms_mat4 {
            self.shader.set_matrix_uniform(uniform.0, uniform.1);
        }
        let mut texture_index = 0;
        for texture in &self.textures {
            self.shader.set_int_uniform(texture.0, texture_index);
            texture.1.bind(texture_index.try_into().unwrap());
            texture_index += 1;
        }
    }
}
