use super::shader::Shader;
use super::texture::Texture;

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

    pub fn bind(&self) {
        self.shader.use_program();
        for texture in &self.textures {
            texture.bind();
        }
    }
}
