use crate::maths::matrix::Mat4f;
use crate::maths::vector::Vect3f;
use crate::maths::vector::Vect4f;
use crate::structure::Structure;
use super::cube;
use super::frame_limiter::FrameLimiter;
use super::material::Material;
use super::mesh::Mesh;
use super::opengl::vertex_objects::Primitive;

pub struct Renderer {
    frame_limiter: FrameLimiter,
    cube_mesh: Mesh,
    interface_mesh: Mesh,
}

impl Renderer {
    pub fn new() -> Self {
        let cube_mesh = {
            let mut material = Material::create("shaders/hello_texture.vert", "shaders/hello_texture.frag");

            let texture = cube::cube_texture([0x40, 0x40, 0x40, 0xFF], [0x80, 0x80, 0x80, 0xFF]);
            material.add_texture(texture);

            cube::cube_mesh(material)
        };

        let interface_mesh = {
            let mut material = Material::create("shaders/hello_vertex.vert", "shaders/hello_color.frag");
            material.add_uniform("uni_color", Vect4f::new([1.0, 1.0, 1.0, 0.5]));

            let positions: Vec<f32> = [
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
            interface_mesh: interface_mesh,
        }
    }

    pub fn render_frame(&mut self, projection_view_matrix: &Mat4f, structure: &Structure) {
        structure.for_each_voxel(|x, y, z| {
            let model_matrix = structure.repere().clone() * Mat4f::translation(Vect3f::new([x as f32, y as f32, z as f32]));
            self.cube_mesh.draw(projection_view_matrix, &model_matrix);
        });

        self.interface_mesh.draw(&Mat4f::identity(), &Mat4f::identity());

        self.frame_limiter.limit();
    }
}
