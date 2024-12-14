use super::super::meshes::material::Material;
use super::super::meshes::mesh::Mesh;
use super::super::opengl::vertex_objects::Primitive;
use crate::warfare::projectile::Projectile;

use super::super::core::color::Color;
use crate::maths::matrix::Mat4f;

pub struct ProjectileRenderer {
    projectile_mesh: Mesh,
}

impl ProjectileRenderer {
    pub fn new() -> Self {
        let projectile_mesh = {
            let mut material = Material::create("shaders/position.vert", "shaders/hello_color.frag");
            material.add_uniform_mat4("uni_model_matrix", Mat4f::identity());
            material.add_uniform_vect4("uni_color", Color::new(0xB0, 0x30, 0x30, 0xFF).into());

            let positions = [
                0.2, 0.0, 0.0,
                0.0, 0.2, 0.0,
                0.0, 0.0, 0.2,

                0.2, 0.0, 0.0,
                0.0, 0.0, 0.2,
                0.0, -0.2, 0.0,

                0.2, 0.0, 0.0,
                0.0, -0.2, 0.0,
                0.0, 0.0, -0.2,

                0.2, 0.0, 0.0,
                0.0, 0.0, -0.2,
                0.0, 0.2, 0.0,

                -0.5, 0.0, 0.0,
                0.0, 0.2, 0.0,
                0.0, 0.0, 0.2,

                -0.5, 0.0, 0.0,
                0.0, 0.0, 0.2,
                0.0, -0.2, 0.0,

                -0.5, 0.0, 0.0,
                0.0, -0.2, 0.0,
                0.0, 0.0, -0.2,

                -0.5, 0.0, 0.0,
                0.0, 0.0, -0.2,
                0.0, 0.2, 0.0,
            ].to_vec();

            let mut mesh = Mesh::create(Primitive::Triangles, false, material);
            mesh.set_positions_3d(&positions);
            mesh
        };

        Self {
            projectile_mesh: projectile_mesh,
        }
    }

    pub fn render(&mut self, projection_view_matrix: &Mat4f, projectiles: &Vec<Projectile>) {
        for projectile in projectiles {
            let direction = projectile.velocity().normalize();
            let yaw = f32::atan2(direction[1], direction[0]);
            let pitch = -f32::asin(direction[2]);
            let model_matrix = Mat4f::translation(projectile.position()) * Mat4f::rotation_around_z(yaw) * Mat4f::rotation_around_y(pitch);
            self.projectile_mesh.set_uniform_matrix("uni_model_matrix", &model_matrix);
            self.projectile_mesh.draw(&projection_view_matrix);
        }
    }
}
