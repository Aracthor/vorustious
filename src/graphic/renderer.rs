use crate::maths::boxes::Box3f;
use crate::maths::matrix::Mat4f;
use crate::maths::vector::Vect2f;
use crate::maths::vector::Vect3f;
use crate::maths::vector::Vect4f;
use crate::editor::Editor;
use crate::physics::body::Body;
use crate::voxels::catalog::VoxelCatalog;
use crate::voxels::voxel::TextureType;
use crate::warfare::projectile::Projectile;
use super::core::color::Color;
use super::cube;
use super::editor_renderer::EditorRenderer;
use super::frame_limiter::FrameLimiter;
use super::meshes::material::Material;
use super::meshes::mesh::Mesh;
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

    debug_cube_mesh: Mesh,
    gizmo_mesh: Mesh,
    show_bodies: bool,
    show_boxes: bool,
    show_gizmo: bool,
    show_octtree: bool,
}

fn positions_from_box(min: Vect3f, max: Vect3f) -> Vec<f32> {
    // TODO being able to use glDrawElements would avoid duplicate vertices here.
    [
        min[0], min[1], min[2],
        min[0], min[1], max[2],
        min[0], min[1], max[2],
        min[0], max[1], max[2],
        min[0], max[1], max[2],
        min[0], max[1], min[2],
        min[0], max[1], min[2],
        min[0], min[1], min[2],

        min[0], min[1], min[2],
        max[0], min[1], min[2],
        max[0], min[1], min[2],
        max[0], max[1], min[2],
        max[0], max[1], min[2],
        min[0], max[1], min[2],

        min[0], min[1], max[2],
        max[0], min[1], max[2],
        max[0], min[1], max[2],
        max[0], max[1], max[2],
        max[0], max[1], max[2],
        min[0], max[1], max[2],

        max[0], min[1], min[2],
        max[0], min[1], max[2],
        max[0], max[1], min[2],
        max[0], max[1], max[2],
    ].to_vec()
}

impl Renderer {
    pub fn new(window_width: f32, window_height: f32, fov: f32, z_near: f32, z_far: f32) -> Self {
        let projection_matrix = {
            let aspect = window_width / window_height;
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

        let debug_cube = {
            let mut material = Material::create("shaders/position.vert", "shaders/hello_color.frag");
            material.add_uniform_mat4("uni_model_matrix", Mat4f::identity());
            material.add_uniform_vect4("uni_color", Vect4f::zero());

            Mesh::create(Primitive::Lines, true, material)
        };

        let gizmo_mesh = {
            let mut material = Material::create("shaders/colored.vert", "shaders/colored.frag");
            material.add_uniform_mat4("uni_model_matrix", Mat4f::identity());

            let size = 10.0;
            let positions = [
                0.0, 0.0, 0.0,
                size, 0.0, 0.0,
                0.0, 0.0, 0.0,
                0.0, size, 0.0,
                0.0, 0.0, 0.0,
                0.0, 0.0, size,
            ].to_vec();
            let colors = [
                1.0, 0.0, 0.0, 1.0,
                1.0, 0.0, 0.0, 1.0,
                0.0, 1.0, 0.0, 1.0,
                0.0, 1.0, 0.0, 1.0,
                0.0, 0.0, 1.0, 1.0,
                0.0, 0.0, 1.0, 1.0,
            ].to_vec();

            let mut mesh = Mesh::create(Primitive::Lines, false, material);
            mesh.set_positions_3d(&positions);
            mesh.set_colors(&colors);
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

            debug_cube_mesh: debug_cube,
            gizmo_mesh: gizmo_mesh,
            show_bodies: true,
            show_boxes: false,
            show_gizmo: false,
            show_octtree: false,
        }
    }

    pub fn toggle_debug_bodies(&mut self) {
        self.show_bodies = !self.show_bodies;
    }

    pub fn toggle_debug_boxes(&mut self) {
        self.show_boxes = !self.show_boxes;
    }

    pub fn toggle_gizmo(&mut self) {
        self.show_gizmo = !self.show_gizmo;
    }

    pub fn toggle_octtree(&mut self) {
        self.show_octtree = !self.show_octtree;
    }

    pub fn render_frame(&mut self, view_matrix: Mat4f, bodies: Vec<&Body>, projectiles: &Vec<Projectile>, editor: Option<&Editor>) {
        let projection_view_matrix = self.projection_matrix.clone() * view_matrix;
        for body in bodies {
            if self.show_bodies {
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

            if self.show_boxes {
                let body_box = body.structure().get_box();
                let positions = positions_from_box(body_box.min(), body_box.max());
                self.debug_cube_mesh.set_positions_3d(&positions);
                self.debug_cube_mesh.set_uniform_vector("uni_color", Vect4f::new([0.0, 1.0, 0.0, 1.0]));
                self.debug_cube_mesh.set_uniform_matrix("uni_model_matrix", body.repere());
                self.debug_cube_mesh.draw(&projection_view_matrix);

                let global_box = {
                    let mut global_box = Box3f::new();
                    for corner in body_box.corners() {
                        global_box.add(body.repere().clone() * corner);
                    }
                    global_box
                };
                let positions = positions_from_box(global_box.min(), global_box.max());
                self.debug_cube_mesh.set_positions_3d(&positions);
                self.debug_cube_mesh.set_uniform_vector("uni_color", Vect4f::new([1.0, 1.0, 1.0, 1.0]));
                self.debug_cube_mesh.set_uniform_matrix("uni_model_matrix", &Mat4f::identity());
                self.debug_cube_mesh.draw(&projection_view_matrix);
            }

            if self.show_gizmo {
                self.gizmo_mesh.set_uniform_matrix("uni_model_matrix", body.repere());
                self.gizmo_mesh.draw(&projection_view_matrix)
            }

            if self.show_octtree {
                body.structure().octtree().walk(&mut |oct, _| {
                    let recenter = Vect3f::all(0.5);
                    let min = Vect3f::new([oct.min()[0] as f32, oct.min()[1] as f32, oct.min()[2] as f32]) + recenter;
                    let max = Vect3f::new([oct.max()[0] as f32, oct.max()[1] as f32, oct.max()[2] as f32]) + recenter;
                    let positions = positions_from_box(min, max);
                    let color = match oct.extent()[0] {
                        32 => Vect4f::new([0.0, 1.0, 1.0, 1.0]),
                        16 => Vect4f::new([0.0, 1.0, 0.0, 1.0]),
                        8 => Vect4f::new([1.0, 1.0, 0.0, 1.0]),
                        4 => Vect4f::new([1.0, 0.0, 1.0, 1.0]),
                        2 => Vect4f::new([1.0, 0.0, 1.0, 1.0]),
                        1 => Vect4f::new([1.0, 1.0, 1.0, 1.0]),
                        _ => todo!(),
                    };
                    self.debug_cube_mesh.set_positions_3d(&positions);
                    self.debug_cube_mesh.set_uniform_vector("uni_color", color);
                    self.debug_cube_mesh.set_uniform_matrix("uni_model_matrix", body.repere());
                    self.debug_cube_mesh.draw(&projection_view_matrix);
                });
            }
        }

        for projectile in projectiles {
            let direction = projectile.velocity().normalize();
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

        let frame_time_info = self.frame_limiter.frame_time();
        let min_time_ms = frame_time_info.min * 1000.0;
        let max_time_ms = frame_time_info.max * 1000.0;
        let average_time_ms = frame_time_info.average * 1000.0;
        let text = format!("Frame time: {average_time_ms:.2} ms ({min_time_ms:.2}..{max_time_ms:.2})");
        let size = Vect2f::new([12.0, 12.0]);
        let position = Vect2f::new([10.0, self.resolution[1] - size[1] - 10.0]);
        self.text_drawer.add_text_to_draw(text.as_str(), position, size);
        self.text_drawer.draw(&self.ui_projection_matrix);

        self.frame_limiter.limit();
    }
}
