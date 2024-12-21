use std::f32::consts::FRAC_PI_4;
use std::f32::consts::TAU;

use super::super::core::color::Color;
use super::super::meshes::mesh::Mesh;
use super::super::meshes::material::Material;
use super::super::opengl::vertex_objects::Primitive;
use crate::maths::matrix::Mat4f;

pub struct ReticleRenderer {
    mesh: Mesh,
}

fn cross_vertex_positions(distance: f32, radius: f32) -> Vec<f32> {
    let circle_steps = 24;

    let mut positions = Vec::<f32>::default();
    positions.push(distance);
    positions.push(radius);
    positions.push(0.0);
    for i in 0..circle_steps {
        let angle = TAU * i as f32 / circle_steps as f32;
        positions.push(distance);
        positions.push(radius * f32::cos(angle));
        positions.push(radius * f32::sin(angle));
        positions.push(distance);
        positions.push(radius * f32::cos(angle));
        positions.push(radius * f32::sin(angle));
    }
    positions.push(distance);
    positions.push(radius);
    positions.push(0.0);
    for i in 0..4 {
        let angle = TAU * i as f32 / 4.0 + FRAC_PI_4;
        positions.push(distance);
        positions.push(radius * f32::cos(angle));
        positions.push(radius * f32::sin(angle));
        positions.push(distance);
        positions.push(radius * f32::cos(angle) * 3.0 / 4.0);
        positions.push(radius * f32::sin(angle) * 3.0 / 4.0);
    }

    positions
}

impl ReticleRenderer {
    pub fn new() -> Self {
        let reticle_mesh = {
            let mut material = Material::create("shaders/position.vert", "shaders/hello_color.frag");
            material.add_uniform_mat4("uni_model_matrix", Mat4f::identity());
            material.add_uniform_vect4("uni_color", Color::new(0xFF, 0xFF, 0xFF, 0x80).into());

            let positions = {
                let first_circle_distance = 10.0;
                let first_circle_radius = 4.0;
                let second_circle_distance = 20.0;
                let second_circle_radius = 2.0;

                let mut positions = Vec::<f32>::default();
                positions.extend(cross_vertex_positions(first_circle_distance, first_circle_radius));
                positions.extend(cross_vertex_positions(second_circle_distance, second_circle_radius));
                positions
            };

            let mut mesh = Mesh::create(Primitive::Lines, false, material);
            mesh.set_positions_3d(&positions);
            mesh
        };

        Self {
            mesh: reticle_mesh,
        }
    }

    pub fn render(&mut self, projection_view_matrix: &Mat4f, player_repere: &Mat4f) {
        self.mesh.set_uniform_matrix("uni_model_matrix", &player_repere);
        self.mesh.draw(projection_view_matrix);
    }
}
