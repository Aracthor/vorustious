use super::shader::Shader;

pub struct Material {
    shader: Shader,
}

impl Material {
    pub fn create(vertex_file_name: &str, fragment_file_name: &str) -> Material {
        Material {
            shader: Shader::create_shader_program(vertex_file_name, fragment_file_name),
        }
    }

    pub fn bind(&self) {
        self.shader.use_program();
    }
}
