use super::cube;
use super::super::meshes::material::Material;
use super::super::meshes::mesh::Mesh;
use super::super::opengl::vertex_objects::Primitive;
use crate::maths::boxes::Box3f;
use crate::maths::matrix::Mat4f;
use crate::maths::vector::Vect3f;
use crate::maths::vector::Vect4f;
use crate::physics::body::Body;
use crate::voxels::catalog::VoxelCatalog;

pub struct BodyRenderer {
    cube_mesh: Mesh,
    voxel_catalog: VoxelCatalog,

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

impl BodyRenderer {
    pub fn new() -> Self {
        let cube_mesh = cube::cube_mesh();

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

        Self {
            cube_mesh: cube_mesh,
            voxel_catalog: VoxelCatalog::create(),

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

    pub fn render(&mut self, projection_view_matrix: &Mat4f, bodies: Vec<&Body>) {
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
    }
}
