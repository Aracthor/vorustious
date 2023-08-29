use super::gl_check::gl_check;

use crate::maths::matrix::Mat4f;

pub struct Shader {
    vertex_shader: gl::types::GLuint,
    fragment_shader: gl::types::GLuint,
    program: gl::types::GLuint,
}

fn read_file_content(file_name: &str) -> String {
    match std::fs::read_to_string(file_name) {
        Ok(result) => result,
        Err(error) => panic!("Error reading shader file {file_name} : {error}"),
    }
}

unsafe fn compile_shader(shader_file: &str, shader_code: &str, shader_type: gl::types::GLenum) -> gl::types::GLuint {
    let shader = gl::CreateShader(shader_type);
    gl::ShaderSource(shader, 1, &shader_code.as_bytes().as_ptr().cast(), &shader_code.len().try_into().unwrap());
    gl::CompileShader(shader);
    gl_check();

    let mut success = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    if success == 0 {
        const CAPACITY: i32 = 0x1000;
        let mut log_len:i32 = 0;
        let mut error_data: Vec<u8> = Vec::with_capacity(CAPACITY.try_into().unwrap());
        gl::GetShaderInfoLog(shader, CAPACITY, &mut log_len, error_data.as_mut_ptr().cast());
        error_data.set_len(log_len.try_into().unwrap());
        panic!("Error compiling shader '{shader_file}': {}", String::from_utf8_lossy(&error_data));
    }
    shader
}

impl Shader {
    pub fn create_shader_program(vertex_file_name: &str, fragment_file_name: &str) -> Shader {
        let vertex_file_content = read_file_content(vertex_file_name);
        let vertex_shader = unsafe { compile_shader(vertex_file_name, &vertex_file_content, gl::VERTEX_SHADER) };
        let fragment_file_content = read_file_content(fragment_file_name);
        let fragment_shader = unsafe { compile_shader(fragment_file_name, &fragment_file_content, gl::FRAGMENT_SHADER) };

        let program = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
            gl_check();

            let mut success = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                const CAPACITY: i32 = 0x1000;
                let mut error_data: Vec<u8> = Vec::with_capacity(CAPACITY.try_into().unwrap());
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(program, CAPACITY, &mut log_len, error_data.as_mut_ptr().cast());
                error_data.set_len(log_len.try_into().unwrap());
                panic!("Error linking program between '{vertex_file_name}' and '{fragment_file_name}' : {}", String::from_utf8_lossy(&error_data));
            }
        }

        Shader {
            vertex_shader: vertex_shader,
            fragment_shader: fragment_shader,
            program: program,
        }
    }

    pub fn set_matrix_uniform(&self, uniform_name: &str, matrix: &Mat4f) {
        let mut location_name: Vec<gl::types::GLchar> = Default::default();
        location_name.reserve(uniform_name.len() + 1);
        for c in uniform_name.as_bytes() {
            location_name.push((*c).try_into().unwrap());
        }
        location_name.push(0);

        unsafe {
            let location = gl::GetUniformLocation(self.program, location_name.as_ptr());
            assert!(location >= 0, "invalid uniform {uniform_name}");
            gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.data_as_ptr());
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
            gl::DeleteShader(self.vertex_shader);
            gl::DeleteShader(self.fragment_shader);
        }
    }
}
