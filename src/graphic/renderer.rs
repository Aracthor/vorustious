use crate::maths::matrix::Mat4f;
use crate::editor::Editor;
use crate::voxels::body::Body;
use crate::voxels::catalog::VoxelCatalog;
use crate::voxels::voxel::TextureType;
use crate::warfare::projectile::Projectile;
use super::core::color::Color;
use super::cube;
use super::editor_renderer::EditorRenderer;
use super::frame_limiter::FrameLimiter;
use super::material::Material;
use super::mesh::Mesh;
use super::opengl::vertex_objects::Primitive;
use super::opengl::texture::Texture;

pub struct Renderer {
    projection_matrix: Mat4f,
    frame_limiter: FrameLimiter,
    cube_mesh: Mesh,
    projectile_mesh: Mesh,
    interface_mesh: Mesh,
    voxel_catalog: VoxelCatalog,
    editor_renderer: EditorRenderer,
}

impl Renderer {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        let projection_matrix = {
            let fov = 80.0_f32.to_radians();
            let aspect = window_width / window_height;
            let z_near = 0.1;
            let z_far = 1000.0;
            Mat4f::perspective(fov, aspect, z_near, z_far)
        };
        let cube_mesh = {
            let mut material = Material::create("shaders/voxel.vert", "shaders/voxel.frag");

            material.add_instance_data_buffer("instance_position", 3);
            material.add_instance_data_buffer("instance_texture_index", 1);
            material.add_instance_data_buffer("instance_damage", 1);

            let hull_texture = cube::cube_texture(Color::new(0x40, 0x40, 0x40, 0xFF), Color::new(0x80, 0x80, 0x80, 0xFF));
            material.add_texture(&format!("voxel_texture[{}]", TextureType::Hull as i32), hull_texture);
            let core_texture = cube::cube_texture(Color::new(0x80, 0x80, 0x80, 0xFF), Color::new(0x80, 0x80, 0xFF, 0xFF));
            material.add_texture(&format!("voxel_texture[{}]", TextureType::Core as i32), core_texture);

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
            projection_matrix: projection_matrix,
            frame_limiter: FrameLimiter::new(60.0),
            cube_mesh: cube_mesh,
            projectile_mesh: projectile_mesh,
            interface_mesh: interface_mesh,
            voxel_catalog: VoxelCatalog::create(),
            editor_renderer: EditorRenderer::new(),
        }
    }

    pub fn render_frame(&mut self, view_matrix: Mat4f, bodies: &Vec<Body>, projectiles: &Vec<Projectile>, editor: Option<&Editor>) {
        let projection_view_matrix = self.projection_matrix.clone() * view_matrix;
        for body in bodies {
            let mut instance_positions: Vec<f32> = Default::default();
            let mut instance_texture_indices: Vec<i32> = Default::default();
            let mut instance_damages: Vec<f32> = Default::default();
            body.structure().for_each_voxel(|coords, voxel| {
                let descriptor = self.voxel_catalog.get_descriptor(voxel.id);
                instance_positions.push(coords[0] as f32);
                instance_positions.push(coords[1] as f32);
                instance_positions.push(coords[2] as f32);
                instance_texture_indices.push(descriptor.texture_type as i32);
                instance_damages.push(1.0 - voxel.life / descriptor.max_life);
            });
            let instance_count = instance_damages.len().try_into().unwrap();
            self.cube_mesh.set_instanced_data(0, &instance_positions);
            self.cube_mesh.set_instanced_data(1, &instance_texture_indices);
            self.cube_mesh.set_instanced_data(2, &instance_damages);
            self.cube_mesh.draw_instanced(instance_count, &projection_view_matrix, body.repere());
        }

        for projectile in projectiles {
            let model_matrix = Mat4f::translation(projectile.position());
            self.projectile_mesh.draw(&projection_view_matrix, &model_matrix);
        }

        if editor.is_some() {
            self.editor_renderer.render(&projection_view_matrix, &mut self.cube_mesh, editor.unwrap());
        }

        self.interface_mesh.draw(&Mat4f::identity(), &Mat4f::identity());

        self.frame_limiter.limit();
    }
}
