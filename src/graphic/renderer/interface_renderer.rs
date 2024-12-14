use super::super::core::color::Color;
use super::super::meshes::mesh::Mesh;
use super::super::meshes::material::Material;
use super::super::opengl::vertex_objects::Primitive;
use crate::maths::matrix::Mat4f;

pub struct InterfaceRenderer {
    interface_mesh: Mesh,
}

impl InterfaceRenderer {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        let interface_mesh = {
            let mut material = Material::create("shaders/hello_vertex.vert", "shaders/hello_color.frag");
            material.add_uniform_vect4("uni_color", Color::new(0xFF, 0xFF, 0xFF, 0x80).into());

            let half_width = window_width / 2.0;
            let half_height = window_height / 2.0;
            let reticle_center_size = 5.0;
            let reticle_line_size = 10.0;
            let positions = [
                half_width - reticle_center_size - reticle_line_size, half_height,
                half_width - reticle_center_size,  half_height,
                half_width + reticle_center_size + reticle_line_size, half_height,
                half_width + reticle_center_size,  half_height,
                half_width, half_height - reticle_center_size - reticle_line_size,
                half_width, half_height - reticle_center_size,
                half_width, half_height + reticle_center_size + reticle_line_size,
                half_width, half_height + reticle_center_size,
            ].to_vec();

            let mut mesh = Mesh::create(Primitive::Lines, false, material);
            mesh.set_positions_2d(&positions);
            mesh
        };

        Self {
            interface_mesh: interface_mesh,
        }
    }

    pub fn draw(&self, ui_projection_matrix: &Mat4f) {
        self.interface_mesh.draw(ui_projection_matrix);
    }
}
