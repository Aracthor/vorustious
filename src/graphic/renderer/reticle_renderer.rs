use std::f32::consts::FRAC_PI_4;
use std::f32::consts::TAU;

use super::super::core::color::Color;
use super::super::meshes::mesh::Mesh;
use super::super::meshes::material::Material;
use super::super::opengl::vertex_objects::Primitive;
use crate::maths::matrix::Mat4f;
use crate::maths::vector::Vect3f;
use crate::warfare::ship::Ship;

enum ReticleMode {
    None,
    Ship,
    Weapons,
}

impl ReticleMode {
    fn toggle(&mut self) {
        *self = match *self {
            Self::None => Self::Ship,
            Self::Ship => Self::Weapons,
            Self::Weapons => Self::None,
        }
    }
}


pub struct ReticleRenderer {
    mesh: Mesh,
    mode: ReticleMode,
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
        positions.push(radius * f32::cos(angle) * 2.0 / 3.0);
        positions.push(radius * f32::sin(angle) * 2.0 / 3.0);
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
                let first_circle_distance = 1.0;
                let first_circle_radius = 0.5;
                let second_circle_distance = 2.0;
                let second_circle_radius = 0.2;

                let mut positions = Vec::<f32>::default();
                positions.extend(cross_vertex_positions(first_circle_distance, first_circle_radius));
                positions.extend(cross_vertex_positions(second_circle_distance, second_circle_radius));
                positions
            };

            let mut mesh = Mesh::create(Primitive::Lines, material, false);
            mesh.set_positions_3d(&positions);
            mesh
        };

        Self {
            mesh: reticle_mesh,
            mode: ReticleMode::Ship,
        }
    }

    pub fn toggle_reticle_mode(&mut self) {
        self.mode.toggle();
    }

    pub fn render(&mut self, projection_view_matrix: &Mat4f, ship: &Ship) {
        let ship_repere = ship.body().repere();
        match self.mode {
            ReticleMode::None => {},
            ReticleMode::Ship => {
                let scale_transformation = Mat4f::scale(Vect3f::new([10.0, 5.0, 5.0]));
                let reticle_matrix = ship_repere.clone() * scale_transformation;
                self.mesh.set_uniform_matrix("uni_model_matrix", &reticle_matrix);
                self.mesh.draw(projection_view_matrix);
            },
            ReticleMode::Weapons => {
                let scale_transformation = Mat4f::scale(Vect3f::new([10.0, 2.0, 2.0]));
                for weapon in ship.weapons() {
                    let weapon_transformation = Mat4f::translation(weapon.0) * scale_transformation.clone();
                    let reticle_matrix = ship_repere.clone() * weapon_transformation;
                    self.mesh.set_uniform_matrix("uni_model_matrix", &reticle_matrix);
                    self.mesh.draw(projection_view_matrix);
                }
            },
        }
    }
}
