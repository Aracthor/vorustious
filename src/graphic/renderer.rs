use crate::maths::matrix::Mat4f;
use crate::maths::vector::Vect3f;
use crate::maths::vector::Vect3i;
use crate::maths::vector::Vect4f;
use crate::projectile::Projectile;
use crate::voxels::structure::Structure;
use super::cube;
use super::frame_limiter::FrameLimiter;
use super::material::Material;
use super::mesh::Mesh;
use super::opengl::vertex_objects::Primitive;

pub struct Renderer {
    frame_limiter: FrameLimiter,
    cube_mesh: Mesh,
    ghost_cube_mesh: Mesh,
    projectile_mesh: Mesh,
    interface_mesh: Mesh,
}

impl Renderer {
    pub fn new() -> Self {
        let cube_mesh = {
            let mut material = Material::create("shaders/hello_texture.vert", "shaders/hello_texture.frag");

            let texture = cube::cube_texture([0x40, 0x40, 0x40, 0xFF], [0x80, 0x80, 0x80, 0xFF]);
            material.add_texture(texture);

            material.add_uniform_f32("uni_alpha", 1.0);
            cube::cube_mesh(material)
        };
        let ghost_cube_mesh = {
            let mut material = Material::create("shaders/hello_texture.vert", "shaders/hello_texture.frag");

            let texture = cube::cube_texture([0x40, 0x40, 0x40, 0xFF], [0x80, 0x80, 0x80, 0xFF]);
            material.add_texture(texture);

            material.add_uniform_f32("uni_alpha", 0.5);
            cube::cube_mesh(material)
        };

        let projectile_mesh = {
            let mut material = Material::create("shaders/hello_vertex.vert", "shaders/hello_color.frag");
            material.add_uniform_vect4("uni_color", Vect4f::new([0.8, 0.2, 0.2, 1.0]));

            let positions = [
                0.5, 0.0, 0.0,
                0.0, 0.5, 0.0,
                0.0, 0.0, 0.5,

                0.5, 0.0, 0.0,
                0.0, 0.0, 0.5,
                0.0, -0.5, 0.0,

                0.5, 0.0, 0.0,
                0.0, -0.5, 0.0,
                0.0, 0.0, -0.5,

                0.5, 0.0, 0.0,
                0.0, 0.0, -0.5,
                0.0, 0.5, 0.0,

                -0.5, 0.0, 0.0,
                0.0, 0.5, 0.0,
                0.0, 0.0, 0.5,

                -0.5, 0.0, 0.0,
                0.0, 0.0, 0.5,
                0.0, -0.5, 0.0,

                -0.5, 0.0, 0.0,
                0.0, -0.5, 0.0,
                0.0, 0.0, -0.5,

                -0.5, 0.0, 0.0,
                0.0, 0.0, -0.5,
                0.0, 0.5, 0.0,
            ].to_vec();

            Mesh::create(positions, vec![], Primitive::Triangles, material)
        };

        let interface_mesh = {
            let mut material = Material::create("shaders/hello_vertex.vert", "shaders/hello_color.frag");
            material.add_uniform_vect4("uni_color", Vect4f::new([1.0, 1.0, 1.0, 0.5]));

            let positions = [
                -0.07, 0.0, 0.0,
                -0.02, 0.0, 0.0,
                0.07, 0.0, 0.0,
                0.02, 0.0, 0.0,
                0.0, -0.07, 0.0,
                0.0, -0.02, 0.0,
                0.0, 0.07, 0.0,
                0.0, 0.02, 0.0,
            ].to_vec();

            Mesh::create(positions, vec![], Primitive::Lines, material)
        };

        Renderer {
            frame_limiter: FrameLimiter::new(60.0),
            cube_mesh: cube_mesh,
            ghost_cube_mesh: ghost_cube_mesh,
            projectile_mesh: projectile_mesh,
            interface_mesh: interface_mesh,
        }
    }

    pub fn render_frame(&mut self, projection_view_matrix: &Mat4f, structure: &Structure, projectiles: &Vec<Projectile>, ghost_position: Option<Vect3i>) {
        structure.for_each_voxel(|x, y, z| {
            let model_matrix = structure.repere().clone() * Mat4f::translation(Vect3f::new([x as f32, y as f32, z as f32]));
            self.cube_mesh.draw(projection_view_matrix, &model_matrix);
        });

        for projectile in projectiles {
            let model_matrix = Mat4f::translation(projectile.position());
            self.projectile_mesh.draw(projection_view_matrix, &model_matrix);
        }

        if ghost_position.is_some() {
            let position = ghost_position.unwrap();
            let model_matrix = structure.repere().clone() * Mat4f::translation(Vect3f::new([position[0] as f32, position[1] as f32, position[2] as f32]));
            self.ghost_cube_mesh.draw(projection_view_matrix, &model_matrix);
        }

        self.interface_mesh.draw(&Mat4f::identity(), &Mat4f::identity());

        self.frame_limiter.limit();
    }
}
