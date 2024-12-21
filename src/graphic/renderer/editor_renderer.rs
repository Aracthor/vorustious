use crate::editor::Editor;
use crate::maths::matrix::Mat4f;
use crate::voxels::catalog::VoxelCatalog;
use super::super::core::color::Color;
use super::super::meshes::material::Material;
use super::super::meshes::mesh::Mesh;
use super::super::opengl::vertex_objects::Primitive;
use super::cube;

pub struct EditorRenderer {
    voxel_catalog: VoxelCatalog,
    cube_mesh: Mesh,
    plane_x: Mesh,
    plane_y: Mesh,
    plane_z: Mesh,
}

impl EditorRenderer {
    pub fn new() -> Self {
        let plane_x = {
            let positions = [
                0.0, -1000.0, -1000.0,
                0.0, -1000.0,  1000.0,
                0.0,  1000.0,  1000.0,
                0.0,  1000.0,  1000.0,
                0.0, -1000.0, -1000.0,
                0.0,  1000.0, -1000.0,
            ].to_vec();
            let mut material = Material::create("shaders/hello_vertex.vert", "shaders/hello_color.frag");
            material.add_uniform_vect4("uni_color", Color::new(0xFF, 0x00, 0x00, 0x40).into());
            let mut mesh = Mesh::create(Primitive::Triangles, material, false);
            mesh.set_positions_3d(&positions);
            mesh
        };

        let plane_y = {
            let positions = [
                -1000.0, 0.0, -1000.0,
                -1000.0, 0.0,  1000.0,
                 1000.0, 0.0,  1000.0,
                 1000.0, 0.0,  1000.0,
                 -1000.0, 0.0, -1000.0,
                 1000.0, 0.0, -1000.0,
            ].to_vec();
            let mut material = Material::create("shaders/hello_vertex.vert", "shaders/hello_color.frag");
            material.add_uniform_vect4("uni_color", Color::new(0x00, 0xFF, 0x00, 0x40).into());
            let mut mesh = Mesh::create(Primitive::Triangles, material, false);
            mesh.set_positions_3d(&positions);
            mesh
        };

        let plane_z = {
            let positions = [
                -1000.0, -1000.0, 0.0,
                -1000.0,  1000.0, 0.0,
                 1000.0,  1000.0, 0.0,
                 1000.0,  1000.0, 0.0,
                 -1000.0, -1000.0, 0.0,
                 1000.0, -1000.0, 0.0,
            ].to_vec();
            let mut material = Material::create("shaders/hello_vertex.vert", "shaders/hello_color.frag");
            material.add_uniform_vect4("uni_color", Color::new(0x00, 0x00, 0xFF, 0x40).into());
            let mut mesh = Mesh::create(Primitive::Triangles, material, false);
            mesh.set_positions_3d(&positions);
            mesh
        };

        Self {
            voxel_catalog: VoxelCatalog::create(),
            cube_mesh: cube::cube_mesh(0.5),
            plane_x: plane_x,
            plane_y: plane_y,
            plane_z: plane_z,
        }
    }

    pub fn render(&mut self, projection_view_matrix: &Mat4f, editor: &Editor) {
        if editor.voxel_position.is_some() {
            let position = editor.voxel_position.unwrap();
            let instance_position = vec![position[0] as f32, position[1] as f32, position[2] as f32];
            let instance_texture_index = vec![self.voxel_catalog.get_descriptor(editor.voxel_id).texture_type as i32];
            let instance_damage = vec![0.0];
            self.cube_mesh.set_instanced_data(0, &instance_position);
            self.cube_mesh.set_instanced_data(1, &instance_texture_index);
            self.cube_mesh.set_instanced_data(2, &instance_damage);
            self.cube_mesh.draw_instanced(1, &projection_view_matrix);
        }
        if editor.symetry_x {
            self.plane_x.draw(&projection_view_matrix);
        }
        if editor.symetry_y {
            self.plane_y.draw(&projection_view_matrix);
        }
        if editor.symetry_z {
            self.plane_z.draw(&projection_view_matrix);
        }
    }
}