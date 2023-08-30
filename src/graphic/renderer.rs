use crate::maths::matrix::Mat4f;
use crate::maths::vector::Vect3f;
use crate::graphic::cube;
use crate::graphic::frame_limiter::FrameLimiter;
use crate::graphic::material::Material;
use crate::graphic::mesh::Mesh;
use crate::structure::Structure;

pub struct Renderer {
    frame_limiter: FrameLimiter,
    cube_mesh: Mesh,
}

impl Renderer {
    pub fn new() -> Self {
        let cube_mesh = {
            let mut material = Material::create("shaders/hello_texture.vert", "shaders/hello_texture.frag");

            let texture = cube::cube_texture([0x40, 0x40, 0x40, 0xFF], [0x80, 0x80, 0x80, 0xFF]);
            material.add_texture(texture);

            cube::cube_mesh(material)
        };

        Renderer {
            frame_limiter: FrameLimiter::new(60.0),
            cube_mesh: cube_mesh,
        }
    }

    pub fn render_frame(&mut self, projection_matrix: &Mat4f, view_matrix: &Mat4f, structure: &Structure) {
        structure.for_each_voxel(|x, y, z| {
            let model_matrix = Mat4f::translation(Vect3f::new([x as f32, y as f32, z as f32]));
            self.cube_mesh.draw(&projection_matrix, view_matrix, &model_matrix);
        });

        self.frame_limiter.limit();
    }
}
