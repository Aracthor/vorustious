use crate::maths::matrix::Mat4f;
use crate::maths::vector::Vect2f;
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
use super::text_drawer::TextDrawer;

pub struct Renderer {
    resolution: Vect2f,
    projection_matrix: Mat4f,
    ui_projection_matrix: Mat4f,
    frame_limiter: FrameLimiter,
    cube_mesh: Mesh,
    projectile_mesh: Mesh,
    interface_mesh: Mesh,
    text_drawer: TextDrawer,
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
        let ui_projection_matrix = Mat4f::orthographic(0.0, window_width, 0.0, window_height);
        let cube_mesh = {
            let mut material = Material::create("shaders/voxel.vert", "shaders/voxel.frag");

            material.add_instance_data_buffer("instance_position", 3);
            material.add_instance_data_buffer("instance_texture_index", 1);
            material.add_instance_data_buffer("instance_damage", 1);
            material.add_uniform_mat4("uni_model_matrix", Mat4f::identity());

            let hull_texture = cube::cube_texture(Color::new(0x40, 0x40, 0x40, 0xFF), Color::new(0x80, 0x80, 0x80, 0xFF));
            material.add_texture(&format!("voxel_texture[{}]", TextureType::LightHull as i32), hull_texture);
            let hull_texture = cube::cube_texture(Color::new(0x20, 0x20, 0x20, 0xFF), Color::new(0x30, 0x30, 0x30, 0xFF));
            material.add_texture(&format!("voxel_texture[{}]", TextureType::DarkHull as i32), hull_texture);
            let core_texture = cube::cube_texture(Color::new(0x80, 0x80, 0x80, 0xFF), Color::new(0x80, 0x80, 0xFF, 0xFF));
            material.add_texture(&format!("voxel_texture[{}]", TextureType::Core as i32), core_texture);
            let canon_texture = cube::circle_texture(Color::new(0x80, 0x80, 0x80, 0xFF), Color::new(0x20, 0x20, 0x20, 0xFF));
            material.add_texture(&format!("voxel_texture[{}]", TextureType::Canon as i32), canon_texture);

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
            let mut material = Material::create("shaders/projectile.vert", "shaders/hello_color.frag");
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

        Renderer {
            resolution: Vect2f::new([window_width, window_height]),
            projection_matrix: projection_matrix,
            ui_projection_matrix: ui_projection_matrix,
            frame_limiter: FrameLimiter::new(60.0),
            cube_mesh: cube_mesh,
            projectile_mesh: projectile_mesh,
            interface_mesh: interface_mesh,
            text_drawer: TextDrawer::create(),
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
            self.cube_mesh.set_uniform_matrix("uni_model_matrix", body.repere());
            self.cube_mesh.draw_instanced(instance_count, &projection_view_matrix);
        }

        for projectile in projectiles {
            let direction = projectile.movement().normalize();
            let yaw = f32::atan2(direction[1], direction[0]);
            let pitch = -f32::asin(direction[2]);
            let model_matrix = Mat4f::translation(projectile.position()) * Mat4f::rotation_around_z(yaw) * Mat4f::rotation_around_y(pitch);
            self.projectile_mesh.set_uniform_matrix("uni_model_matrix", &model_matrix);
            self.projectile_mesh.draw(&projection_view_matrix);
        }

        if editor.is_some() {
            self.editor_renderer.render(&projection_view_matrix, &mut self.cube_mesh, editor.unwrap());
        }

        self.interface_mesh.draw(&self.ui_projection_matrix);

        let elapsed_time_ms = self.frame_limiter.elapsed_time_secs() * 1000.0;
        let text = format!("Frame time: {elapsed_time_ms} ms");
        let size = Vect2f::new([12.0, 12.0]);
        let position = Vect2f::new([10.0, self.resolution[1] - size[1] - 10.0]);
        self.text_drawer.add_text_to_draw(text.as_str(), position, size);
        self.text_drawer.draw(&self.ui_projection_matrix);

        self.frame_limiter.limit();
    }
}
