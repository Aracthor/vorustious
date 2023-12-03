use crate::maths::matrix::Mat4f;
use crate::maths::vector::Vect3i;
use crate::projectile::Projectile;
use crate::voxels::body::Body;
use crate::voxels::voxel::TextureType;
use super::core::color::Color;
use super::cube;
use super::frame_limiter::FrameLimiter;
use super::material::Material;
use super::mesh::Mesh;
use super::opengl::vertex_objects::Primitive;
use super::opengl::texture::Texture;

pub struct Renderer {
    frame_limiter: FrameLimiter,
    cube_mesh: Mesh,
    projectile_mesh: Mesh,
    interface_mesh: Mesh,
}

impl Renderer {
    pub fn new() -> Self {
        let cube_mesh = {
            let mut material = Material::create("shaders/voxel.vert", "shaders/voxel.frag");

            material.add_instance_data_buffer("instance_position", 3);
            material.add_instance_data_buffer("instance_texture_index", 1);
            material.add_instance_data_buffer("instance_damage", 1);

            let hull_texture = cube::cube_texture(Color::new(0x40, 0x40, 0x40, 0xFF), Color::new(0x80, 0x80, 0x80, 0xFF));
            material.add_texture(&format!("voxel_texture[{}]", <TextureType as Into<i32>>::into(TextureType::Hull)), hull_texture);
            let core_texture = cube::cube_texture(Color::new(0x80, 0x80, 0x80, 0xFF), Color::new(0x80, 0x80, 0xFF, 0xFF));
            material.add_texture(&format!("voxel_texture[{}]", <TextureType as Into<i32>>::into(TextureType::Core)), core_texture);

            let damage_texture = {
                let b = Color::black();
                let t = Color::transparent();
                let texture_pixels: Vec<Color> = [
                    t,  b,  t,  t,  t,  t,  t,  b,
                    t,  b,  t,  t,  t,  b,  b,  t,
                    t,  b,  t,  t,  b,  t,  t,  b,
                    t,  t,  b,  t,  b,  b,  b,  b,
                    t,  t,  b,  b,  b,  t,  t,  t,
                    t,  b,  t,  b,  t,  b,  t,  b,
                    b,  t,  t,  t,  t,  b,  t,  b,
                    t,  t,  t,  t,  t,  t,  b,  b,
                ].to_vec();
                Texture::create(8, 8, texture_pixels)
            };
            material.add_texture("damage_texture", damage_texture);

            material.add_uniform_f32("uni_alpha", 1.0);
            cube::cube_mesh(material)
        };

        let projectile_mesh = {
            let mut material = Material::create("shaders/hello_vertex.vert", "shaders/hello_color.frag");
            material.add_uniform_vect4("uni_color", Color::new(0xB0, 0x30, 0x30, 0xFF).into());

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

            Mesh::create(positions, None, Primitive::Triangles, material)
        };

        let interface_mesh = {
            let mut material = Material::create("shaders/hello_vertex.vert", "shaders/hello_color.frag");
            material.add_uniform_vect4("uni_color", Color::new(0xFF, 0xFF, 0xFF, 0x80).into());

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

            Mesh::create(positions, None, Primitive::Lines, material)
        };

        Renderer {
            frame_limiter: FrameLimiter::new(60.0),
            cube_mesh: cube_mesh,
            projectile_mesh: projectile_mesh,
            interface_mesh: interface_mesh,
        }
    }

    pub fn render_frame(&mut self, projection_view_matrix: &Mat4f, body: &Body, projectiles: &Vec<Projectile>, ghost_position: Option<Vect3i>) {
        let mut instance_positions: Vec<f32> = Default::default();
        let mut instance_texture_indices: Vec<i32> = Default::default();
        let mut instance_damages: Vec<f32> = Default::default();
        body.structure().for_each_voxel(|x, y, z, voxel| {
            instance_positions.push(x as f32);
            instance_positions.push(y as f32);
            instance_positions.push(z as f32);
            instance_texture_indices.push(voxel.texture_type.into());
            instance_damages.push(1.0 - voxel.life / voxel.max_life);
        });
        let instance_count = instance_damages.len().try_into().unwrap();
        self.cube_mesh.set_instanced_data(0, &instance_positions);
        self.cube_mesh.set_instanced_data(1, &instance_texture_indices);
        self.cube_mesh.set_instanced_data(2, &instance_damages);
        self.cube_mesh.draw_instanced(instance_count, projection_view_matrix, body.repere());

        for projectile in projectiles {
            let model_matrix = Mat4f::translation(projectile.position());
            self.projectile_mesh.draw(projection_view_matrix, &model_matrix);
        }

        if ghost_position.is_some() {
            let position = ghost_position.unwrap();
            let instance_position = vec![position[0] as f32, position[1] as f32, position[2] as f32];
            let instance_texture_index = vec![0];
            let instance_damage = vec![0.0];
            self.cube_mesh.set_instanced_data(0, &instance_position);
            self.cube_mesh.set_instanced_data(1, &instance_texture_index);
            self.cube_mesh.set_instanced_data(2, &instance_damage);
            self.cube_mesh.set_uniform_f32("uni_alpha", 0.5);
            self.cube_mesh.draw_instanced(1, projection_view_matrix, body.repere());
            self.cube_mesh.set_uniform_f32("uni_alpha", 1.0);
        }

        self.interface_mesh.draw(&Mat4f::identity(), &Mat4f::identity());

        self.frame_limiter.limit();
    }
}
